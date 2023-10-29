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

const AGED_BRIE: &str = "Aged Brie";
const BACKSTAGE_PASS: &str = "Backstage passes to a TAFKAL80ETC concert";
const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            let mut item = &mut self.items[i];

            match &item.name[..] {
                SULFURAS => { },
                AGED_BRIE | BACKSTAGE_PASS => {
                    update_sell_in(item);
                    update_quality(item, 1);
                    if item.name == BACKSTAGE_PASS {
                        if item.sell_in < 11 {
                            update_quality(item, 1);
                        }

                        if item.sell_in < 6 {
                            update_quality(item, 1);
                        }
                    }
                },
                _ => {
                    update_sell_in(item);
                    update_quality(item, -1);
                }
            }

            if item.sell_in < 0 {
                if item.name == AGED_BRIE {
                    update_quality(item, 1);
                } else {
                    if item.name == BACKSTAGE_PASS {
                        update_quality(item, -item.quality);
                    } else {
                        update_quality(item, -1);
                    }
                }
            }
        }
    }
}

pub fn update_quality(item: &mut Item, number_of_quality: i32) {
    item.quality = item.quality + number_of_quality;
    if item.quality > 50 {
        item.quality = 50;
    }
    if item.quality < 0 {
        item.quality = 0;
    }
}

pub fn update_sell_in(item: &mut Item) {
    item.sell_in = item.sell_in - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let normal_aged_brie = Item::new(AGED_BRIE, 10, 20);
        let expired_aged_brie = Item::new(AGED_BRIE, 0, 20);
        let items = vec![normal_aged_brie, expired_aged_brie];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(21, rose.items[0].quality, "Aged Brie quality always increasing by 1, every day passed.");
        assert_eq!(22, rose.items[1].quality, "Aged Brie quality increasing by 2, after due date reached.");
    }

    #[test]
    pub fn sulfuras_never_sold_never_decrease_in_quality() {
        let items = vec![Item::new(SULFURAS, 10, 50)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(10, rose.items[0].sell_in, "sell_in is static in Sulfuras.");
        assert_eq!(50, rose.items[0].quality, "quality is static in Sulfuras.");
    }

    #[test]
    pub fn backstage_pass_quality_check() {
        let backstage_pass_with_gt_10_days_sell_in = Item::new(BACKSTAGE_PASS, 15, 20);
        let backstage_pass_with_eq_10_days_sell_in = Item::new(BACKSTAGE_PASS, 10, 20);
        let backstage_pass_with_lt_10_days_gte_5_days_sell_in = Item::new(BACKSTAGE_PASS, 9, 20);
        let backstage_pass_with_eq_5_days_sell_in = Item::new(BACKSTAGE_PASS, 5, 20);
        let backstage_pass_with_lt_5_days_gt_1_days_sell_in = Item::new(BACKSTAGE_PASS, 3, 20);
        let backstage_pass_with_0_day_sell_in = Item::new(BACKSTAGE_PASS, 0, 20);

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
        let normal_aged_brie = Item::new(AGED_BRIE, 10, 50);
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

