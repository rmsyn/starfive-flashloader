use std::io::Write;

// Change if sector address should be relative (starts at 0), or absolute (starts at 0x2100_0000)
const BASE_ADDR: usize = 0;

fn main() -> Result<(), std::io::Error> {
    let mut f = std::fs::File::create("sectors.rs")?;

    f.write_all(b"use super::FlashSector;\n\n")?;
    f.write_all(b"pub(crate) const SECTORS_LEN: usize = 0x400;\n\n")?;
    f.write_all(b"pub(crate) const SECTORS: [FlashSector; SECTORS_LEN] = [\n".as_ref())?;

    for i in 0usize..1024usize {
        let addr: usize = i.saturating_mul(0x1000).saturating_add(BASE_ADDR);
        f.write_all(format!("\tFlashSector::create(0x{addr:06x}),\n").as_bytes())?;
    }

    f.write_all(b"];\n".as_ref())?;

    Ok(())
}
