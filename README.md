# picoserde-wasm-bindgen

Based on https://github.com/not-fl3/nanoserde and https://github.com/cloudflare/serde-wasm-bindgen

## Example:

```rust
use picoserde_wasm_bindgen::{DeJs, SerJs};

#[derive(Clone, Debug, Default, DeJs, SerJs)]
pub struct Property {
    pub name: String,
    #[nserde(default)]
    pub value: String,
    #[nserde(rename = "type")]
    pub ty: String,
}
```

For more examples take a look on [tests](/tests)

## Features support matrix:

| Feature                                         | json   |
| ----------------------------------------------- | ------ |
| serialization                                   | yes    |
| deserialization                                 | yes    |
| container: Struct                               | yes    |
| container: Tuple Struct                         | no     |
| container: Enum                                 | yes    |
| field: `std::collections::HashMap`              | yes    |
| field: `std::vec::Vec`                          | yes    |
| field: `Option`                                 | yes    |
| field: `i*`/`f*`/`String`/`T: De*/Ser*`         | yes    |
| field attribute: `#[picoserde(default)]`        | yes    |
| field attribute: `#[picoserde(rename = "")]`    | yes    |
| field attribute: `#[picoserde(proxy = "")]`     | no     |
| container attribute: `#[picoserde(default)]`    | yes    |
| container attribute: `#[picoserde(rename = "")]`| yes    |
| container attribute: `#[picoserde(proxy = "")]` | yes    |
