use std::error::Error;

mod switch_utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut switch = switch_utils::Switch::new()?;
    switch.write_dword(0xAFB1E6E0, 0x9c9)?;
    switch.write_dword(0xAFB1E6E4, 29)?;

    let mut inventory: [u8; 320] = [0; 320];
    switch.read_bytes(0xAFB1E6E0, &mut inventory, 320)?;

    for i in 0..40 {
        println!(
            "Slot #{}: Item=0x{:04x}, Count={}",
            i,
            u32::from_le_bytes(inventory[i * 8..i * 8 + 4].try_into()?),
            u32::from_le_bytes(inventory[i * 8 + 4..i * 8 + 8].try_into()?)
        )
    }

    Ok(())
}
