use super::Error;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpiNorOpcode {
    /// Write disable
    Wrdi = 0x04,
    /// Write enable
    Wren = 0x06,
    /// Read status register
    Rdsr = 0x05,
    /// Write status register 1 byte
    Wrsr = 0x01,
    /// Read status register 2
    Rdsr2 = 0x3f,
    /// Write status register 2
    Wrsr2 = 0x3e,
    /// Read data bytes (low frequency)
    Read = 0x03,
    /// Read data bytes (high frequency)
    ReadFast = 0x0b,
    /// Read data bytes (Dual Output SPI)
    Read112 = 0x3b,
    /// Read data bytes (Dual I/O SPI)
    Read122 = 0xbb,
    /// Read data bytes (Quad Output SPI)
    Read114 = 0x6b,
    /// Read data bytes (Quad I/O SPI)
    Read144 = 0xeb,
    /// Read data bytes (Octal Output SPI)
    Read118 = 0x8b,
    /// Read data bytes (Octal I/O SPI)
    Read188 = 0xcb,
    /// Page program (up to 256 bytes)
    Pp = 0x02,
    /// Quad page program
    Pp114 = 0x32,
    /// Quad page program
    Pp144 = 0x38,
    /// Octal page program
    Pp118 = 0x82,
    /// Octal page program
    Pp188 = 0xc2,
    /// Erase 4KiB block
    Be4k = 0x20,
    /// Erase 4KiB block on PMC chips
    Be4kPmc = 0xd7,
    /// Erase 32KiB block
    Be32k = 0x52,
    /// Erase whole flash chip
    ChipErase = 0xc7,
    /// Sector erase (usually 64KiB)
    Se = 0xd8,
    /// Read JEDEC ID
    Rdid = 0x9f,
    /// Read SFDP
    RdSfdp = 0x5a,
    /// Read configuration register
    RdCr = 0x35,
    /// Read flag status register
    RdFsr = 0x70,
    /// Clear flag status register
    ClFsr = 0x50,
    /// Read Extended Address Register
    RdEar = 0xc8,
    /// Write Extended Address Register
    WrEar = 0xc5,
    /// Software Reset Enable
    SRstEn = 0x66,
    /// Software Reset
    SRst = 0x99,
    /// Global Block Unlock
    GBulk = 0x98,
}

impl TryFrom<u8> for SpiNorOpcode {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x04 => Ok(Self::Wrdi),
            0x06 => Ok(Self::Wren),
            0x05 => Ok(Self::Rdsr),
            0x01 => Ok(Self::Wrsr),
            0x3f => Ok(Self::Rdsr2),
            0x3e => Ok(Self::Wrsr2),
            0x03 => Ok(Self::Read),
            0x0b => Ok(Self::ReadFast),
            0x3b => Ok(Self::Read112),
            0xbb => Ok(Self::Read122),
            0x6b => Ok(Self::Read114),
            0xeb => Ok(Self::Read144),
            0x8b => Ok(Self::Read118),
            0xcb => Ok(Self::Read188),
            0x02 => Ok(Self::Pp),
            0x32 => Ok(Self::Pp114),
            0x38 => Ok(Self::Pp144),
            0x82 => Ok(Self::Pp118),
            0xc2 => Ok(Self::Pp188),
            0x20 => Ok(Self::Be4k),
            0xd7 => Ok(Self::Be4kPmc),
            0x52 => Ok(Self::Be32k),
            0xc7 => Ok(Self::ChipErase),
            0xd8 => Ok(Self::Se),
            0x9f => Ok(Self::Rdid),
            0x5a => Ok(Self::RdSfdp),
            0x35 => Ok(Self::RdCr),
            0x70 => Ok(Self::RdFsr),
            0x50 => Ok(Self::ClFsr),
            0xc8 => Ok(Self::RdEar),
            0xc5 => Ok(Self::WrEar),
            0x66 => Ok(Self::SRstEn),
            0x99 => Ok(Self::SRst),
            0x98 => Ok(Self::GBulk),
            _ => Err(Error::UnsupportedOpcode),
        }
    }
}
