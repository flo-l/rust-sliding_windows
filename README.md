sliding_windows
===============

This crate offers an ```Iterator``` adaptor, which yields "sliding windows" over the elements returned by the wrapped iterator.

It's worth to note that it does **not** copy elements, which makes the code relatively performant.

### Example

```rust
extern crate sliding_windows;

use sliding_windows::{IterExt, SlidingWindowStorage};

let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);

for x in (0..5).sliding_windows(&mut storage) {
    println!("{:?}", x);
}

// This outputs:
// [0, 1, 2]
// [1, 2, 3]
// [2, 3, 4]
```

### In other languages

- Ruby: [#each_cons](http://ruby-doc.org/core-2.1.0/Enumerable.html#method-i-each_cons)
- Python: [window](https://docs.python.org/release/2.3.5/lib/itertools-example.html)
- Rust (just for slices): [.windows()](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
