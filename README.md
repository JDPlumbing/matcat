# matcat

A compact, procedural material catalog for simulations.

Instead of maintaining giant lookup tables of materials, **matcat** encodes every material as a 5-byte identifier (`MatCatId`), then **procedurally derives its physical properties**. This allows trillions of possible distinct materials — far more than you could ever hand-author — while still giving deterministic, repeatable results.

---

## Features

- **Compact IDs**  
  `MatCatId` is only 5 bytes (`u8` category, `u16` variant, `u16` grade).  
  Each ID maps deterministically to a unique material.

- **Procedural properties**  
  `props_for(id)` derives a complete `MatProps` struct, covering mechanical, thermal, chemical, and electromagnetic properties.

- **Distance metric**  
  Compare materials in property-space with a Euclidean distance function.

- **Search**  
  `find_closest_material` lets you match a target set of properties against a search space of `MatCatId`s.

- **Blazing fast**  
  Property derivation (`props_for`) takes ~12ns; searching 1000 candidates takes ~17µs (see benches).

---

## Example

```rust
use matcat::{MatCatId, props_for, find_closest_material};

fn main() {
    let copper_id = MatCatId::new(1, 42, 0); // category=1 (Metal), variant=42, grade=0
    let copper_props = props_for(&copper_id);

    println!("Copper-like density: {} kg/m³", copper_props.density);

    // Suppose we want the closest match to a target density
    let target = matcat::materials::MatProps { density: 8000.0, ..Default::default() };
    let candidates: Vec<_> = (0..1000).map(|v| MatCatId::new(1, v, 0)).collect();
    if let Some((id, props)) = find_closest_material(&target, &candidates) {
        println!("Closest material ID: {:?}, density={}", id, props.density);
    }
}
```

---

## Use cases

- Simulation engines that need consistent materials without manual authoring.  
- ECS systems where materials are components.  
- Procedural generation of worlds, objects, and environments.  
- Approximate matching for “what’s the closest material to this set of properties?”

---

## Performance

Benchmarked on a modern x86 CPU:

- `props_for`: ~12 ns  
- `find_closest_material` (1000 candidates): ~17 µs  

---

## License

MIT
