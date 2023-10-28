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
    pub fn normal_item() {
        let items = vec![Item::new("Dexterity Vest", 10, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(19, rose.items[0].quality);
    }

    #[test]
    pub fn normal_item_date_passed() {
        let items = vec![Item::new("Dexterity Vest", 0, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(-1, rose.items[0].sell_in);
        assert_eq!(18, rose.items[0].quality);
    }

    #[test]
    pub fn quality_minimum_at_0() {
        let items = vec![Item::new("Dexterity Vest", 10, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_increase_quality() {
        let items = vec![Item::new("Aged Brie", 10, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(21, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_increase_quality_twice_when_date_passed() {
        let items = vec![Item::new("Aged Brie", 0, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(-1, rose.items[0].sell_in);
        assert_eq!(22, rose.items[0].quality);
    }

    #[test]
    pub fn quality_maximum_at_50() {
        let items = vec![Item::new("Aged Brie", 10, 50)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn sulfuras_never_sold_never_decrease_in_quality() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 10, 50)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(10, rose.items[0].sell_in);
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn back_stage_pass_normal_date() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(14, rose.items[0].sell_in);
        assert_eq!(21, rose.items[0].quality);
    }

    #[test]
    pub fn back_stage_pass_less_than_10_days_and_5_days_or_more() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(22, rose.items[0].quality);
    }

    #[test]
    pub fn back_stage_pass_less_than_5_days() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(4, rose.items[0].sell_in);
        assert_eq!(23, rose.items[0].quality);
    }

    #[test]
    pub fn back_stage_pass_after_concert() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(-1, rose.items[0].sell_in);
        assert_eq!(0, rose.items[0].quality);
    }

    #[ignore]
    #[test]
    pub fn conjured_item() {
        let items = vec![Item::new("Conjured Item", 10, 20)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(18, rose.items[0].quality);
    }
}
