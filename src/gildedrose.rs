use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name != "Aged Brie" && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn normal_item_quality_check() {
        let normal_item = Item::new("Dexterity Vest", 10, 20);
        let expired_normal_item = Item::new("Dexterity Vest", 0, 20);
        let items = vec![normal_item, expired_normal_item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(19, rose.items[0].quality, "Normal item quality always decreasing by 1, every day passed.");
        assert_eq!(18, rose.items[1].quality, "Normal item quality always decreasing by 2, after due date reach.");
    }

    #[test]
    pub fn aged_brie_quality_check() {
        let normal_aged_brie = Item::new("Aged Brie", 10, 20);
        let expired_aged_brie = Item::new("Aged Brie", 0, 20);
        let items = vec![normal_aged_brie, expired_aged_brie];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(21, rose.items[0].quality, "Aged Brie quality always increasing by 1, every day passed.");
        assert_eq!(22, rose.items[1].quality, "Aged Brie quality increasing by 2, after due date reached.");
    }

    #[test]
    pub fn sulfuras_never_sold_never_decrease_in_quality() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 10, 50)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(10, rose.items[0].sell_in, "sell_in is static in Sulfuras.");
        assert_eq!(50, rose.items[0].quality, "quality is static in Sulfuras.");
    }

    #[test]
    pub fn backstage_pass_quality_check() {
        let backstage_pass_with_gt_10_days_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20);
        let backstage_pass_with_eq_10_days_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 20);
        let backstage_pass_with_lt_10_days_gte_5_days_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 9, 20);
        let backstage_pass_with_eq_5_days_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 20);
        let backstage_pass_with_lt_5_days_gt_1_days_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 3, 20);
        let backstage_pass_with_0_day_sell_in = Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 20);

        let items = vec![
            backstage_pass_with_gt_10_days_sell_in,
            backstage_pass_with_eq_10_days_sell_in,
            backstage_pass_with_lt_10_days_gte_5_days_sell_in,
            backstage_pass_with_eq_5_days_sell_in,
            backstage_pass_with_lt_5_days_gt_1_days_sell_in,
            backstage_pass_with_0_day_sell_in,
        ];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(21, rose.items[0].quality, "Backstage pass always increasing in quality by 1 when sell_in more than 10");
        assert_eq!(22, rose.items[1].quality, "Backstage pass always increasing in quality by 2 when sell_in == 10");
        assert_eq!(22, rose.items[2].quality, "Backstage pass always increasing in quality by 2 when 5 < sell_in < 10");
        assert_eq!(23, rose.items[3].quality, "Backstage pass always increasing in quality by 3 when sell_in == 5");
        assert_eq!(23, rose.items[4].quality, "Backstage pass always increasing in quality by 3 when 0 < sell_in < 5");
        assert_eq!(0, rose.items[5].quality, "Backstage pass quality become 0 after due date");
    }

    #[test]
    pub fn quality_limit_check() {
        let normal_aged_brie = Item::new("Aged Brie", 10, 50);
        let normal_item_with_0_quality = Item::new("Dexterity Vest", 10, 0);
        let items = vec![normal_aged_brie, normal_item_with_0_quality];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(50, rose.items[0].quality, "Aged Brie quality never more than 50.");
        assert_eq!(0, rose.items[1].quality, "Any item quality never less than 0.");
    }

    #[test]
    pub fn sell_in_check() {
        let normal_item = Item::new("Dexterity Vest", 10, 20);
        let expired_normal_item = Item::new("Dexterity Vest", 0, 20);
        let items = vec![normal_item, expired_normal_item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in, "sell_in always decreasing every day.");
        assert_eq!(-1, rose.items[1].sell_in, "sell_in can be negative if due date reached.");
    }

    #[ignore]
    #[test]
    pub fn conjured_item() {
        let conjured_item = Item::new("Conjured Mana Cake", 10, 20);
        let expired_conjured_item = Item::new("Conjured Mana Cake", 0, 20);
        let items = vec![conjured_item, expired_conjured_item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(18, rose.items[0].quality, "Conjured item quality always decreasing by 2, every day passed.");
        assert_eq!(16, rose.items[1].quality, "Normal item quality always decreasing by 4, after due date reach.");
    }
}
