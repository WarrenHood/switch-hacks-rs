use std::error::Error;

mod switch_utils;
mod acnh_utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut acnh = acnh_utils::ACNH::new();

    acnh.fill_inventory(&acnh_utils::InventoryItem::Item(0x8a4, 1))?;
    println!("Filled inventory... New inventory: {:#?}", acnh.get_inventory()?);

    acnh.set_inventory(0, &acnh_utils::InventoryItem::Recipe(0x0297))?;
    println!("Added golden axe recipe... New inventory: {:#?}", acnh.get_inventory()?);
    Ok(())
}
