sliding_windows
===============

This crate offers an ```Iterator``` adaptor, which yields "sliding windows" over the elements returned by the wrapped iterator.

It's worth to note that it does **not** copy elements, which makes the code several orders of mignitudes faster than naive copying variants.

As a consequence it violates the Iterator protocol slightly. It is not possible to have two Windows into the data
available at the same time. This is checked during runtime.

The backing storage is a ```Vec```, so this Iterator adaptor is not ideal for very large windows (>20 elements or very huge elements).

I'd happily accept a PR to implement the same functionality with a ```VecDeque``` or similar, see this [issue](https://github.com/flo-l/rust-sliding_windows/issues/2).

### Links

- [Documentation](https://flo-l.github.io/rust-sliding_windows/)
- [crates.io](https://crates.io/crates/sliding_windows)

### Install

Add this to your ```Cargo.toml```.

    [dependencies]
    sliding_windows = "3.0"

### Example

```rust
extern crate sliding_windows;
use sliding_windows::{IterExt, Storage};

fn main() {
    let mut storage: Storage<u32> = Storage::new(3);

    for x in (0..5).sliding_windows(&mut storage) {
        println!("{:?}", x);
    }
}

// This outputs:
// Window[0, 1, 2]
// Window[1, 2, 3]
// Window[2, 3, 4]
```

For more examples please consult the [Documentation](https://flo-l.github.io/rust-sliding_windows/).

### This functionality in other languages/crates

- Ruby: [#each_cons](http://ruby-doc.org/core-2.1.0/Enumerable.html#method-i-each_cons)
- Python: [window](https://docs.python.org/release/2.3.5/lib/itertools-example.html)
- Rust (just for slices): [.windows()](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- Rust (for all Iterators, but copying): [.slide()](https://github.com/slapresta/rust-iterslide)
