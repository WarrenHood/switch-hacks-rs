use std::error::Error;

mod switch_utils;
mod acnh_utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut acnh = acnh_utils::ACNH::new();
    acnh.fill_inventory(0x8a4, 30)?;

    println!("Filled inventory... New inventory: {:#?}", acnh.get_inventory()?);
    acnh.set_inventory_recipe(0, 0x0297)?;
    Ok(())
}
