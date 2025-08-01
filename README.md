## 📦 `rs-unitconv` — A Simple Unit Conversion Library for Rust

> ✨ Lightweight, zero-dependency utility crate for common unit conversions: length, weight, temperature and more.

---

### 🚀 Features

- 📏 **Length**: `cm ↔ inch`, `m ↔ ft`
- ⚖️ **Weight**: `kg ↔ lb`
- 🌡️ **Temperature**: `°C ↔ °F`, `°C ↔ K`
- 💡 Intuitive function names and clean API
- 🧪 Tested and ready for embedded or CLI use
- 🔀 No external dependencies

---

### 📥 Installation

```toml
# Cargo.toml
[dependencies]
rs-unitconv = "0.1.0"
```

### 🧑‍💻Usage
```rust
use rs_unitconv::length::{cm_to_inch, inch_to_cm};
use rs_unitconv::weight::{kg_to_lb, lb_to_kg};
use rs_unitconv::temperature::{c_to_f, f_to_c};

fn main() {
    println!("100 cm = {:.2} in", cm_to_inch(100.0));
    println!("220 lb = {:.2} kg", lb_to_kg(220.0));
    println!("37 °C = {:.2} °F", c_to_f(37.0));
}
```