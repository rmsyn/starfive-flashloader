#![no_std]
#![no_main]
#![deny(clippy::panic)]

//! Define necessary functions for flash loader
//!
//! These are taken from the [ARM CMSIS-Pack documentation]
//!
//! [ARM CMSIS-Pack documentation]: <https://arm-software.github.io/CMSIS_5/Pack/html/algorithmFunc.html>

use core::slice;

extern crate panic_halt;

#[cfg(feature = "visionfive2-12a")]
use jh7110_vf2_12a_pac::{qspi, QSPI};
#[cfg(feature = "visionfive2-13b")]
use jh7110_vf2_13b_pac::{qspi, QSPI};

use riscv::register::mcycle;

mod opcode;
mod sectors;

use opcode::*;
use sectors::*;

const FLASH_MEM: usize = 0x2100_0000;
const FLASH_LEN: usize = 0x40_0000;
const FLASH_END: usize = 0x2140_0000;

const SECTOR_LEN: usize = 0x1000;

const FIFO_MEM: usize = 0x2100_0000;
const FIFO_DEPTH: usize = 0x100;
const FIFO_WIDTH: usize = 0x04;

// JH7110 SoCs have 128M SPI-NOR flash, which requires 27-bit addresses (so 4 bytes)
const ADDR_BYTES: usize = 4;

const TRIGGER_ADDRESS: u32 = 0x00;

const TIMEOUT_MS: u64 = 500;
const TIMEOUT_US: u64 = 500_000;

const PAGE_LEN: usize = 256;
// Empty byte in the SPI-NOR flash
// Implicit in the Linux driver because the temp mod word is initialized to `0xffff_ffff`
const EMPTY: u8 = 0xff;

const VERSION: u16 = 0x0;

pub type Result<T> = core::result::Result<T, Error>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum Error {
    InvalidLen(usize),
    Timeout,
    UnsupportedOpcode,
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
#[link_section = "DeviceData"]
static FlashDevice: FlashDeviceDescription = FlashDeviceDescription {
    vers: VERSION,
    dev_name: [0u8; 128],
    dev_type: 5,
    dev_addr: FLASH_MEM as u32,
    device_size: FLASH_LEN as u32,
    page_size: PAGE_LEN as u32,
    _reserved: 0,
    empty: EMPTY,
    program_time_out: TIMEOUT_MS as u32,
    erase_time_out: TIMEOUT_MS as u32,
    flash_sectors: SECTORS,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct FlashDeviceDescription {
    vers: u16,
    dev_name: [u8; 128],
    dev_type: u16,
    dev_addr: u32,
    device_size: u32,
    page_size: u32,
    _reserved: u32,
    empty: u8,
    program_time_out: u32,
    erase_time_out: u32,
    flash_sectors: [FlashSector; SECTORS_LEN],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct FlashSector {
    size: u32,
    address: u32,
}

impl FlashSector {
    /// Creates a new [FlashSector] with the provided address.
    pub const fn create(address: u32) -> Self {
        Self {
            size: SECTOR_LEN as u32,
            address,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionType {
    Single = 0,
    Dual = 1,
    Quad = 2,
    Octal = 3,
}

#[inline]
fn delay_us(us: u32) {
    let t0 = mcycle::read64();
    let us_64 = u64::from(us);
    let clock = us_64
        .saturating_mul(1_500_000_000)
        .saturating_div(1_000_000u64);
    while mcycle::read64().wrapping_sub(t0) <= clock {}
}

fn wait_for_idle(spi: &qspi::RegisterBlock) -> Result<()> {
    let mut timeout = TIMEOUT_US;
    let mut idle = spi.config.read().idle().bit_is_set() as u8;
    let idle_retries = 3;

    while timeout > 0 && idle < idle_retries {
        delay_us(10);
        timeout = timeout.saturating_sub(10);
        idle = idle.saturating_add(spi.indirect_wr.read().done().bit_is_set() as u8);
    }

    if idle >= idle_retries {
        Ok(())
    } else {
        Err(Error::Timeout)
    }
}

fn wait_for_exec_completion(spi: &qspi::RegisterBlock) -> Result<()> {
    let mut timeout = TIMEOUT_US;
    let mut done = spi.cmd_ctrl.read().in_progress().bit_is_clear();

    while timeout > 0 && !done {
        delay_us(10);
        timeout = timeout.saturating_sub(10);
        done = spi.cmd_ctrl.read().in_progress().bit_is_clear();
    }

    if done {
        Ok(())
    } else {
        Err(Error::Timeout)
    }
}

fn wait_for_wip_clear(spi: &qspi::RegisterBlock) -> Result<()> {
    let mut timeout = TIMEOUT_US;
    let mut done = spi.indirect_wr.read().done().bit_is_set();

    while timeout > 0 && !done {
        delay_us(10);
        timeout = timeout.saturating_sub(10);
        done = spi.indirect_wr.read().done().bit_is_set();
    }

    if done {
        Ok(())
    } else {
        Err(Error::Timeout)
    }
}

fn calc_rdreg(spi: &qspi::RegisterBlock) {
    let inst = InstructionType::Quad as u8;

    spi.rd_instr.modify(|_, w| {
        w.type_instr()
            .variant(inst)
            .type_addr()
            .variant(inst)
            .type_data()
            .variant(inst)
    });
}

fn setup_write(spi: &qspi::RegisterBlock, opcode: SpiNorOpcode) {
    let op = opcode as u8;
    let inst_width = InstructionType::Quad as u8;

    spi.wr_instr.modify(|_, w| {
        w.opcode()
            .variant(op)
            .type_data()
            .variant(inst_width)
            .type_addr()
            .variant(inst_width)
    });

    calc_rdreg(spi);

    spi.wr_completion_ctrl
        .modify(|_, w| w.disable_auto_poll().set_bit());

    spi.size
        .modify(|_, w| w.address().variant(ADDR_BYTES as u8));
}

// Low-level direct write to the AHB FIFO buffer.
//
// Callers must first setup a `write` call by calling `setup_write` with the appropriate write
// opcode.
fn write_to_fifo(spi: &qspi::RegisterBlock, buf: &[u8]) -> Result<()> {
    let buf_len = buf.len();
    if buf_len > FIFO_DEPTH {
        Err(Error::InvalidLen(buf_len))
    } else {
        // SAFETY: we have exclusive access to the AHB FIFO buffer
        //
        // The address and length are guaranteed valid by the TRM memory map.
        //
        // Table 2-4 <https://doc-en.rvspace.org/JH7110/PDF/JH7110_TRM_StarFive_Preliminary_V2.pdf>
        //
        // `FIFO_DEPTH` is taken from the Linux/U-Boot DTS
        let ahb_fifo: &mut [u8] =
            unsafe { slice::from_raw_parts_mut(FIFO_MEM as *mut _, FIFO_DEPTH) };

        spi.indirect_wr.modify(|_, w| w.start().set_bit());

        let buf_len = buf.len();
        let words_len = buf_len.saturating_div(FIFO_WIDTH);
        let words_mod = buf_len % FIFO_WIDTH;

        ahb_fifo
            .chunks_exact_mut(FIFO_WIDTH)
            .take(words_len)
            .zip(buf.chunks_exact(FIFO_WIDTH))
            .for_each(|(dst, src)| dst.copy_from_slice(src));

        if words_mod != 0 {
            let last_word = words_len.saturating_mul(FIFO_WIDTH);
            let last_word_end = last_word.saturating_add(words_mod);
            let last_fill = FIFO_WIDTH.saturating_sub(words_mod);
            let last_word_fill = last_word_end.saturating_add(last_fill);
            let fill_word = [0xffu8; FIFO_WIDTH];

            ahb_fifo[last_word..last_word_end].copy_from_slice(&buf[last_word..last_word_end]);
            ahb_fifo[last_word_end..last_word_fill].copy_from_slice(&fill_word[..last_fill]);
        }

        wait_for_wip_clear(spi)?;

        // Disable interrupt
        spi.irq_mask.reset();

        // Clear indirect completion status
        spi.indirect_wr.modify(|_, w| w.done().set_bit());

        wait_for_idle(spi)
    }
}

fn check_valid_write(address: usize, size: usize) -> i32 {
    // check that the size is not larger than a page
    (check_valid_page(address) != 0
     // check that the end of the write is in valid range
     || !(FLASH_MEM..FLASH_END).contains(&address.saturating_add(size).saturating_sub(1))
     || size > PAGE_LEN) as u8 as i32
}

fn check_valid_page(address: usize) -> i32 {
    // check that the `address` is in valid range
    (!(FLASH_MEM..FLASH_END).contains(&address)
     // check `address` is on a page boundary
     || address % PAGE_LEN != 0) as u8 as i32
}

fn check_valid_sector(address: usize) -> i32 {
    (check_valid_page(address) != 0 && address % SECTOR_LEN != 0) as u8 as i32
}

/// Sets up the device for the flashloader.
///
/// Returns 0 on success, 1 on failure.
///
/// # Safety
///
/// Caller must ensure exclusive access to the QSPI control register.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn Init(_adr: u32, _clk: u32, _fnc: u32) -> i32 {
    // SAFETY: caller has exclusive access to the peripheral, and the memory location is guaranteed
    // valid.
    let spi: &qspi::RegisterBlock = unsafe { &(*QSPI::ptr()) };

    // disable the memory-mapped flash
    spi.config.modify(|_, w| w.enable().clear_bit());

    // Configure the remap address register, no remap
    spi.remap.reset();

    // Disable all interrupts
    spi.irq_mask.reset();

    // Configure SRAM split to 1:1
    spi.sram_partition
        .write(|w| w.size().variant((FIFO_DEPTH.saturating_div(2)) as u32));

    // Load indirect trigger address
    spi.indirect_trigger
        .write(|w| w.address().variant(TRIGGER_ADDRESS));

    // Program read watermark -- 1/2 of the FIFO.
    spi.indirect_rd_watermark.write(|w| {
        w.watermark()
            .variant(FIFO_DEPTH.saturating_mul(FIFO_WIDTH).saturating_div(2) as u32)
    });

    // Program write watermark -- 1/8 of the FIFO.
    spi.indirect_wr_watermark.write(|w| {
        w.watermark()
            .variant(FIFO_DEPTH.saturating_mul(FIFO_WIDTH).saturating_div(8) as u32)
    });

    // Disable direct access controller
    spi.config.modify(|_, w| w.enb_dir_acc_ctrl().set_bit());

    // enable the memory-mapped flash
    spi.config.modify(|_, w| w.enable().set_bit());

    0
}

/// Unitialize the QSPI controller
///
/// Returns 0 on success, 1 on failure.
///
/// # Safety
///
/// Caller must ensure exclusive access to the QSPI control register.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn UnInit(_fnc: u32) -> i32 {
    // SAFETY: caller has exclusive access to the peripheral, and the memory location is guaranteed
    // valid.
    let spi: &qspi::RegisterBlock = unsafe { &(*QSPI::ptr()) };

    // disable the memory-mapped flash
    spi.config.modify(|_, w| w.enable().clear_bit());

    // Disable all interrupts
    spi.irq_mask.reset();

    0
}

/// Programs a page of memory in the flash chip.
///
/// Returns 0 on success, 1 on failure.
///
/// # Safety
///
/// - Caller must ensure exclusive access to the QSPI control register.
/// - `adr` must be in valid range, and on a page boundary
/// - `sz` must be less-than-or-equal to a page size
/// - `buf` must point to contiguous memory that is at least as large as `sz`
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn ProgramPage(adr: u32, sz: u32, buf: *const u8) -> i32 {
    let address = adr as usize;
    let size = sz as usize;

    if check_valid_write(address, size) != 0 {
        1
    } else {
        // SAFETY: caller has exclusive access to the peripheral, and the memory location is guaranteed
        // valid.
        let spi: &qspi::RegisterBlock = unsafe { &(*QSPI::ptr()) };

        // Setup controller for a ProgramPage write
        setup_write(spi, SpiNorOpcode::Pp);

        spi.indirect_wr_start_addr
            .write(|w| w.address().variant(adr));
        spi.indirect_wr_bytes.write(|w| w.bytes().variant(sz));

        // SAFETY: given the guarantees made by the caller, `buf` should point to valid, contiguous
        // memory.
        //
        // The checks in `check_valid_address` also ensures we satisfy valid address + size invariants.
        let buffer = unsafe { slice::from_raw_parts(buf, size) };

        write_to_fifo(spi, buffer).is_err() as u8 as i32
    }
}

/// Erase the sector at the given address in flash
///
/// Returns 0 on success, 1 on failure.
///
/// # Safety
///
/// - Caller must ensure exclusive access to the QSPI control register.
/// - `adr` must be in valid range, and on a sector boundary.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn EraseSector(adr: u32) -> i32 {
    if check_valid_sector(adr as usize) != 0 {
        1
    } else {
        // SAFETY: caller has exclusive access to the peripheral, and the memory location is guaranteed
        // valid.
        let spi: &qspi::RegisterBlock = unsafe { &(*QSPI::ptr()) };

        // Set the command address for the sector to erase
        spi.cmd_address.write(|w| w.address().variant(adr));

        // Erase command for a single sector is
        // the 0xD7 (Block Erase 4k PMC) command, followed by a 4-byte address
        spi.cmd_ctrl.modify(|_, w| {
            w.addr_en()
                .set_bit()
                .opcode()
                .variant(SpiNorOpcode::Be4kPmc as u8)
                .execute()
                .set_bit()
        });

        (wait_for_exec_completion(spi).is_err() || wait_for_idle(spi).is_err()) as u8 as i32
    }
}
