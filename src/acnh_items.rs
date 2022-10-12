use std::error::Error;
use std::io::Read;
use std::path::Path;

#[derive(PartialEq)]
pub enum ItemType {
    Item,
    Recipe,
}

#[derive(Debug, Clone)]
pub enum Item {
    Item {
        item_id: u32,
        i_name: String,
        eng_name: String,
        color: String,
    },
    Recipe {
        recipe_id: u32,
        i_name: String,
        eng_name: String,
    },
}

impl Item {
    fn get_id(&self) -> u32 {
        match self {
            Item::Item { item_id, i_name, eng_name, color } => *item_id,
            Item::Recipe { recipe_id, i_name, eng_name } => *recipe_id,
        }
    }

    fn get_type(&self) -> ItemType {
        match self {
            Item::Item { item_id, i_name, eng_name, color } => ItemType::Item,
            Item::Recipe { recipe_id, i_name, eng_name } => ItemType::Recipe,
        }
    }
}

pub struct AcnhItems {
    items: Vec<Item>,
}

impl AcnhItems {
    pub fn new() -> Self {
        let mut acnh_items = AcnhItems { items: Vec::new() };
        acnh_items
            .load_items()
            .expect("Couldn't load items from csv");
        acnh_items
    }

    pub fn get_item_by_id(&self, query_id: u32) -> Option<Item> {
        self.items.iter().filter(|i| i.get_type() == ItemType::Item && i.get_id() == query_id).map(|i| i.clone()).nth(0)
    }

    pub fn get_recipe_by_id(&self, query_id: u32) -> Option<Item> {
        self.items.iter().filter(|i| i.get_type() == ItemType::Recipe && i.get_id() == query_id).map(|i| i.clone()).nth(0)
    }

    pub fn find_item(&self, query: &str) -> Option<Item> {
        self.find_items(query).iter().map(|i| i.clone()).nth(0)
    }

    pub fn find_items(&self, query: &str) -> Vec<Item> {
        self.items.iter().filter(|item| {
            let item_description = format!("{:?}", item).to_lowercase();
            for kw in query.split(" ") {
                if !item_description.contains(&kw.to_lowercase()) {
                    return false;
                }
            }
            true
        }).map(|i| i.clone()).collect()
    }

    fn load_items(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Somehow bundle these csv files
        self.load_items_file(Path::new("./csv/items.csv"), ItemType::Item)?;
        self.load_items_file(Path::new("./csv/recipes.csv"), ItemType::Recipe)?;
        Ok(())
    }

    fn load_items_file<P: AsRef<Path>>(
        &mut self,
        filepath: P,
        item_type: ItemType,
    ) -> Result<(), Box<dyn Error>> {
        let mut f = std::fs::File::open(filepath)?;
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents)?;
        let file_contents = file_contents;

        let mut headers = Vec::<String>::new();
        let mut line_nr = 0;
        file_contents.split("\n").for_each(|x| {
            let cols = x.split(";").map(|c| c.trim());
            if line_nr == 0 {
                cols.for_each(|c| {
                    if c.len() > 0 {
                        headers.push(c.into())
                    }
                });
            } else {
                let cols: Vec<&str> = cols.collect();
                if cols.len() == headers.len() {
                    match item_type {
                        ItemType::Item => self.items.push(Item::Item {
                            item_id: u32::from_str_radix(
                                cols[headers.iter().position(|c| c == "id").unwrap()],
                                16,
                            )
                            .unwrap(),
                            eng_name: cols[headers.iter().position(|c| c == "eng").unwrap()].into(),
                            i_name: cols[headers.iter().position(|c| c == "iName").unwrap()].into(),
                            color: cols[headers.iter().position(|c| c == "color").unwrap()].into(),
                        }),
                        ItemType::Recipe => self.items.push(Item::Recipe {
                            recipe_id: u32::from_str_radix(
                                cols[headers.iter().position(|c| c == "id").unwrap()],
                                16,
                            )
                            .unwrap(),
                            eng_name: cols[headers.iter().position(|c| c == "eng").unwrap()].into(),
                            i_name: cols[headers.iter().position(|c| c == "iName").unwrap()].into(),
                        }),
                    }
                }
            }

            line_nr += 1;
        });

        Ok(())
    }
}
