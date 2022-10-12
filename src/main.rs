use std::error::Error;

mod switch_utils;
mod acnh_utils;
mod acnh_items;

fn main() -> Result<(), Box<dyn Error>> {
    let mut acnh = acnh_utils::ACNH::new();
    let acnh_items = acnh_items::AcnhItems::new();

    acnh.fill_inventory(&acnh_items.find_item("99,000 bells").unwrap(), 1)?;
    println!("Filled inventory... New inventory: {:#?}", acnh.get_inventory(&acnh_items)?);

    acnh.set_inventory(0, &acnh_items.find_item("Golden axe tool recipe").unwrap(), 1)?;
    println!("Added golden axe recipe... New inventory: {:#?}", acnh.get_inventory(&acnh_items)?);
    Ok(())
}
