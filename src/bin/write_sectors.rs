use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let mut f = std::fs::File::create("sectors.rs")?;

    f.write_all(b"use super::FlashSector;\n\n")?;
    f.write_all(b"pub(crate) const SECTORS_LEN: usize = 0x400;\n\n")?;
    f.write_all(b"pub(crate) const SECTORS: [FlashSector; SECTORS_LEN] = [\n".as_ref())?;

    for i in (0x2100_0000..0x2100_0000 + 0x40_0000).step_by(0x1000) {
        f.write_all(format!("\tFlashSector::create(0x{i:06x}),\n").as_bytes())?;
    }

    f.write_all(b"];\n".as_ref())?;

    Ok(())
}
