use crate::categories::Category;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: &'static str,
    pub category: Category,
    pub utomid_ids: Vec<(u8, u8)>, // compact form: element id + count
}

impl Variant {
    pub fn new(name: &'static str, category: Category, utomid_ids: Vec<(u8, u8)>) -> Self {
        Self { name, category, utomid_ids }
    }
}

/// Example variants
pub fn example_variants() -> Vec<Variant> {
    vec![
        Variant::new("Steel", Category::Metal, vec![(26, 1)]), // Iron base
        Variant::new("PVC", Category::Plastic, vec![(6, 2), (1, 3), (17, 1)]), // simplified
        Variant::new("Oak", Category::Wood, vec![(6, 6), (1, 10), (8, 5)]), // cellulose-ish
    ]
}
