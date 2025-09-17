#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Metal,
    Plastic,
    Wood,
    Ceramic,
    Composite,
    Other(String),
}

impl Category {
    pub fn name(&self) -> &str {
        match self {
            Category::Metal => "Metal",
            Category::Plastic => "Plastic",
            Category::Wood => "Wood",
            Category::Ceramic => "Ceramic",
            Category::Composite => "Composite",
            Category::Other(s) => s,
        }
    }
}
