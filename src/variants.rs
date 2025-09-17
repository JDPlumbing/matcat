use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Variants (per category)
pub static VARIANT_MAP: Lazy<HashMap<(u8, u16), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Plastics
    m.insert((2, 1), "PVC");
    m.insert((2, 2), "Polyethylene");
    // Metals
    m.insert((1, 1), "Steel");
    m.insert((1, 2), "Copper");
    m
});
