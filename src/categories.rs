use std::collections::HashMap;
use once_cell::sync::Lazy;

/// High-level categories (u8)
pub static CATEGORY_MAP: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "Metal");
    m.insert(2, "Plastic");
    m.insert(3, "Wood");
    m.insert(4, "Composite");
    m
});
