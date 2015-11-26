sliding_windows
===============

This crate offers an ```Iterator``` adaptor, which yields "sliding windows" over the elements returned by the wrapped iterator.

It's worth to note that it does **not** copy elements, which makes the code relatively performant.

### Links

- [Documentation](https://flo-l.github.io/rust-sliding_windows/)
- [crates.io](https://crates.io/crates/sliding_windows)

### Install

Add this to your ```Cargo.toml```.

    [dependencies]
    sliding_windows = "1.0"

### Example

```rust
extern crate sliding_windows;
use sliding_windows::{IterExt, Storage};

let mut storage: Storage<u32> = Storage::new(3);

for x in (0..5).sliding_windows(&mut storage) {
    println!("{:?}", x);
}

// This outputs:
// [0, 1, 2]
// [1, 2, 3]
// [2, 3, 4]
```

### This functionality in other languages

- Ruby: [#each_cons](http://ruby-doc.org/core-2.1.0/Enumerable.html#method-i-each_cons)
- Python: [window](https://docs.python.org/release/2.3.5/lib/itertools-example.html)
- Rust (just for slices): [.windows()](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
