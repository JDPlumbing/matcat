use crate::{categories::Category, variants::Variant};

#[derive(Debug, Clone)]
pub struct MaterialConfidence {
    pub variant: Variant,
    pub confidence: f32, // 0.0–1.0
}

#[derive(Debug, Clone)]
pub struct Material {
    pub category: Category,
    pub candidates: Vec<MaterialConfidence>,
}

impl Material {
    pub fn new(category: Category, candidates: Vec<MaterialConfidence>) -> Self {
        Self { category, candidates }
    }

    pub fn most_likely(&self) -> Option<&Variant> {
        self.candidates
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .map(|mc| &mc.variant)
    }
}
