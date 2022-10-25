use std::error::Error;

mod switch_utils;
mod acnh_utils;
mod acnh_items;

use eframe::{egui::{self, Button}, emath::Align, epaint::Color32};

struct ACNHHax {
    acnh_items: acnh_items::AcnhItems,
    acnh: acnh_utils::ACNH,
    inventory: Vec<acnh_utils::InventoryItem>,
    current_frame: u8,
    current_query: String,
    current_amount: u32,
    is_recipe: bool,
    bulk_items: bool,
    results: Vec<acnh_items::Item>,
}

impl ACNHHax {
    fn update_inventory(&mut self) {
        self.inventory = self.acnh.get_inventory(&self.acnh_items).unwrap();
    }

    fn update_results(&mut self) {
        if !self.bulk_items {
            self.results = self.acnh_items.find_items(&self.current_query);
            if self.is_recipe {
                self.results = self.results.iter().filter(|x| x.get_type() == acnh_items::ItemType::Recipe).map(|x| x.clone()).collect();
            }
            else {
                self.results = self.results.iter().filter(|x| x.get_type() == acnh_items::ItemType::Item).map(|x| x.clone()).collect();
            }
        }
        else {
            self.results.clear();
        }
    }
}

impl Default for ACNHHax {
    fn default() -> Self {
        let acnh_items = acnh_items::AcnhItems::new();
        let mut acnh = acnh_utils::ACNH::new(); 
        let inventory = acnh.get_inventory(&acnh_items).unwrap();
        Self { 
            acnh_items, acnh,  inventory, current_frame: 0, current_query: String::new(),
            current_amount: 1, is_recipe: false, bulk_items: false, results: Vec::new()
        }
    }
}

impl eframe::App for ACNHHax {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.current_frame = (self.current_frame + 1) % 5;
        if self.current_frame == 0 {
            self.update_inventory();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                ui.label("Item to add");
                let mut update_results = false;
                if self.bulk_items {
                    update_results = ui.text_edit_multiline(&mut self.current_query).changed();
                }
                else {
                    update_results = ui.text_edit_singleline(&mut self.current_query).changed();
                }

                
                
                ui.add(egui::Slider::new(&mut self.current_amount, 1..=30).text("Amount"));
                update_results |= ui.checkbox(&mut self.is_recipe, "Is recipe?").changed();
                update_results |= ui.checkbox(&mut self.bulk_items, "Is item list?").changed();


                if update_results {
                    self.update_results();
                }
            });
            
            if self.results.len() > 0 {
                ui.label("Current results:");
                egui::ScrollArea::vertical().max_height(32.0).id_source("results_scroll_area").show(ui, |ui| {
                    let mut i = 0;
                    for result in self.results.iter() {
                        ui.label(result.to_string());
                        i += 1;
                        if i >= 40 {
                            ui.label(format!("... ({} more results)", self.results.len() - 40).to_string());
                            break;
                        }
                    }
                });
            }
            

            ui.horizontal(|ui| {
                if ui.button("Fill inventory").clicked() {
                    if self.bulk_items {
                        let queries: Vec<&str> = self.current_query.split("\n").collect();
                        let mut slot = 0;
                        for query in queries {
                            let item = match self.is_recipe {
                                true => self.acnh_items.find_recipe(query),
                                false => self.acnh_items.find_item(query),
                            };

                            match item {
                                Some(item) => {
                                    self.acnh.set_inventory(slot, &item, self.current_amount).unwrap();
                                },
                                None => {},
                            };
                            slot += 1;
                            if slot >= 40 {
                                break;
                            }
                        }
                    }
                    else {
                        let item = match self.is_recipe {
                            true => self.acnh_items.find_recipe(&self.current_query),
                            false => self.acnh_items.find_item(&self.current_query),
                        };
    
    
                        match item {
                            Some(item) => {
                                self.acnh.fill_inventory(&item, self.current_amount).unwrap();
                            },
                            None => {},
                        };
                    }
                }
    
                if ui.button("Clear inventory").clicked() {
                    self.acnh.clear_inventory().unwrap();
                }
            });

            ui.separator();
            ui.spacing();

            ui.label("Inventory (click to add)");
            ui.spacing();

            ui.horizontal(|ui| {
                ui.add_space(250.0);


                egui::Grid::new("inventory_grid").min_col_width(10.0).show(ui, |ui| {
                    for row in 0..4 {
                        for col in 0..10 {
                            let inv_item = &self.inventory[row*10 + col];
                            let fill_color: Color32 = match &inv_item.item {
                                acnh_items::Item::Item { item_id, i_name, eng_name, color } => {
                                    if *item_id == 0xfffe {
                                        Color32::BLACK
                                    }
                                    else {
                                        Color32::DARK_GREEN
                                    }
                                },
                                acnh_items::Item::Recipe { recipe_id, i_name, eng_name } => Color32::from_rgb(100, 100, 0),
                            };

                            let inv_cell = Button::new(format!("{:02}", row*10 + col)).fill(fill_color);
                            if ui.add(inv_cell).on_hover_text(inv_item.to_string()).clicked() && !self.bulk_items {
                                
                                let item = match self.is_recipe {
                                    true => self.acnh_items.find_recipe(&self.current_query),
                                    false => self.acnh_items.find_item(&self.current_query),
                                };
    
                                match item {
    
                                    Some(item) => {
                                        self.acnh.set_inventory((row*10 + col) as u32, &item, self.current_amount).unwrap();
                                    },
                                    None => {},
                                    
                                };
                            }
                        }
                        ui.end_row();
                    }
                });
            });

            
            




        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("ACNH USB Hax", options, Box::new(|_| Box::new(ACNHHax::default())));
    Ok(())
}
