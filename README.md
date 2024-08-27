# Lubricant

> Powerful rust toolset

## Quick Start

```bash
# (optional) cargo new
cargo install lubricant
```

```rust
// src/main.rs
use lubricant
fn main(){
    let new_vec = lubricant::array::map(vec![1,2,3], |item|{item*2});
    assert_eq!(new_vec[0], 2);
    assert_eq!(new_vec[1], 4);
    assert_eq!(new_vec[2], 6);
}
```

## Contribution

Welcome PR, but before submit pull request, Please read [Contributing Guidelines]https://github.com/GaoNeng-wWw/lubricant/blob/main/CONTRIBUTING.md)

## LICENSE

MIT
