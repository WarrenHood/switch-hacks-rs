use std::error::Error;

use crate::switch_utils::Switch;
const INVENTORY_OFFSET: u32 = 0xAFB1E6E0;

pub struct ACNH {
    switch: Switch,
}

#[derive(Debug)]
pub enum InventoryItem{
    Item(u32, u32),
    Recipe(u32)
}

impl ACNH {
    pub fn new() -> Self {
        ACNH {
            switch: Switch::new().expect("Could not connect to switch!"),
        }
    }

    pub fn set_inventory_item(&mut self, slot: u32, item_id: u32, count: u32) -> Result<(), Box<dyn Error>> {
        self.switch.write_dword(INVENTORY_OFFSET + slot * 8, item_id)?;
        self.switch.write_dword(INVENTORY_OFFSET + slot * 8 + 4, count - 1)?;
        Ok(())
    }

    pub fn fill_inventory(&mut self, item_id: u32, count: u32) -> Result<(), Box<dyn Error>> {
        for slot in 0..40 {
            self.set_inventory_item(slot, item_id, count)?;
        }
        Ok(())
    }

    pub fn clear_inventory(&mut self) -> Result<(), Box<dyn Error>> {
        self.fill_inventory(0xfffe, 0)
    }

    pub fn get_inventory(&mut self) -> Result<Vec<InventoryItem>, Box<dyn Error>> {
        let mut inventory: [u8; 320] = [0; 320];
        self.switch.read_bytes(0xAFB1E6E0, &mut inventory, 320)?;

        Ok((0..40).map(|i| -> InventoryItem {
            let a = u32::from_le_bytes(inventory[i * 8..i * 8 + 4].try_into().unwrap());
            let b = u32::from_le_bytes(inventory[i * 8 + 4..i * 8 + 8].try_into().unwrap());

            if a == 0x16A2 {
                return InventoryItem::Recipe(b);
            }
            else {
                return InventoryItem::Item (a, b);
            }
        }).collect())
    }
}
