use std::error::Error;

use crate::acnh_items::{self, AcnhItems, Item};
use crate::switch_utils::Switch;
const INVENTORY_OFFSET: u32 = 0xAFB1E6E0;

pub struct ACNH {
    switch: Switch,
}

#[derive(Debug)]
pub struct InventoryItem {
    pub item: Item,
    pub count: u32,
}

impl ToString for InventoryItem {
    fn to_string(&self) -> String {
        match &self.item {
            Item::Item { item_id, i_name, eng_name, color } => {
                return format!("{} x{}", self.item.to_string(), self.count + 1);
            },
            Item::Recipe { recipe_id, i_name, eng_name } => {
                return format!("{}(Recipe)", self.item.to_string());
            },
        }
    }
}

impl ACNH {
    pub fn new() -> Self {
        ACNH {
            switch: Switch::new().expect("Could not connect to switch!"),
        }
    }

    pub fn set_inventory(
        &mut self,
        slot: u32,
        item: &Item,
        count: u32,
    ) -> Result<(), Box<dyn Error>> {
        match item {
            Item::Item {
                item_id,
                i_name: _,
                eng_name: _,
                color: _,
            } => self.set_inventory_item(slot, *item_id, count),
            Item::Recipe {
                recipe_id,
                i_name: _,
                eng_name: _,
            } => self.set_inventory_recipe(slot, *recipe_id),
        }
    }

    pub fn set_inventory_item(
        &mut self,
        slot: u32,
        item_id: u32,
        count: u32,
    ) -> Result<(), Box<dyn Error>> {
        self.switch
            .write_dword(INVENTORY_OFFSET + slot * 8, item_id)?;
        self.switch
            .write_dword(INVENTORY_OFFSET + slot * 8 + 4, count - 1)?;
        Ok(())
    }

    pub fn set_inventory_recipe(
        &mut self,
        slot: u32,
        recipe_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        self.switch
            .write_dword(INVENTORY_OFFSET + slot * 8, 0x16A2)?;
        self.switch
            .write_dword(INVENTORY_OFFSET + slot * 8 + 4, recipe_id)?;
        Ok(())
    }

    pub fn fill_inventory_items(&mut self, item_id: u32, count: u32) -> Result<(), Box<dyn Error>> {
        for slot in 0..40 {
            self.set_inventory_item(slot, item_id, count)?;
        }
        Ok(())
    }

    pub fn fill_inventory_recipes(&mut self, recipe_id: u32) -> Result<(), Box<dyn Error>> {
        for slot in 0..40 {
            self.set_inventory_recipe(slot, recipe_id)?;
        }
        Ok(())
    }

    pub fn fill_inventory(&mut self, item: &Item, count: u32) -> Result<(), Box<dyn Error>> {
        for slot in 0..40 {
            self.set_inventory(slot, &item, count)?;
        }
        Ok(())
    }

    pub fn clear_inventory(&mut self) -> Result<(), Box<dyn Error>> {
        self.fill_inventory_items(0xfffe, 1)
    }

    pub fn get_inventory(&mut self, acnh_items: &AcnhItems) -> Result<Vec<InventoryItem>, Box<dyn Error>> {
        let mut inventory: [u8; 320] = [0; 320];
        self.switch.read_bytes(0xAFB1E6E0, &mut inventory, 320)?;

        Ok((0..40)
            .map(|i| -> InventoryItem {
                let a = u32::from_le_bytes(inventory[i * 8..i * 8 + 4].try_into().unwrap());
                let b = u32::from_le_bytes(inventory[i * 8 + 4..i * 8 + 8].try_into().unwrap());

                if a == 0x16A2 {

                    return InventoryItem {
                        item: acnh_items
                        .get_recipe_by_id(b)
                        .unwrap_or(acnh_items::Item::Recipe {
                            recipe_id: b,
                            i_name: "Unknown".into(),
                            eng_name: "Unknown".into(),
                        }),
                        count: 1,
                    };
                } else {
                    return InventoryItem {
                        item: acnh_items
                        .get_item_by_id(a)
                        .unwrap_or(acnh_items::Item::Item {
                            item_id: a,
                            i_name: "Unknown".into(),
                            eng_name: "Unknown".into(),
                            color: "None".into(),
                        }),
                        count: b,
                    };
                }
            })
            .collect())
    }
}
