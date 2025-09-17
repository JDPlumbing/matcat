# MatCat

**Material Categories & Variants** — a human-friendly layer on top of [`utomid`](https://crates.io/crates/utomid-rs).

---

## 🚀 Features

- Group vague material categories (Metal, Plastic, Wood, …).
- Map specific variants (Steel, PVC, Oak) to chemical definitions (`utomid`-based).
- Handle uncertainty with confidence scores (probabilistic picks).
- Human-friendly input, simulation-ready output.

---

## 📦 Installation

```bash
cargo add matcat
```

---

## 🔍 Example

```rust
use matcat::{Category, Variant, Material, MaterialConfidence};
use matcat::variants::example_variants;

fn main() {
    let variants = example_variants();

    let mat = Material::new(
        Category::Metal,
        vec![
            MaterialConfidence { variant: variants[0].clone(), confidence: 0.8 },
        ],
    );

    println!("Category: {}", mat.category.name());
    println!("Most likely: {:?}", mat.most_likely().unwrap().name);
}
```

---

## 📄 License

MIT © JD Plumbing