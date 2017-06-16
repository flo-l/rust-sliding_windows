//! This crate provides an iterator adaptor that yields sliding windows into the elements of the wrapped Iterator.
//!
//! Note that this adaptor does **NEVER** clone any element for huge speed gains.
//!
//! As a consequence it violates the Iterator protocol slightly. It is not possible to have two Windows into the data
//! available at the same time. This is checked during runtime. If this check fails, the Adaptor panicks in ```next()```.
//! More information can be found in the section [Panics](#panics).
//!
//! There are some options regarding the constructor for Storage, which allow reuse of an allocation.
//! Consult the [docs for Storage](struct.Storage.html) for details.
//!
//! Iterator element type is `Window<'a, Self::Item>`.
//!
//! Note: `Window<'a, Self::Item>` implements `iter()` and `iter_mut()`, which return Iterators over the elements of the `Window`.
//!
//! This iterator is *fused*.
//!
//! # Example:
//!
//! ```
//! use sliding_windows::IterExt;
//! use sliding_windows::Storage;
//!
//! let iter = 0..5;
//! let mut storage: Storage<u32> = Storage::new(3);
//! let windowed_iter = iter.sliding_windows(&mut storage);
//! let output: Vec<Vec<u32>> = windowed_iter.map(|x| x.iter().map(|&x| x).collect()).collect();
//! let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];
//!
//! assert_eq!(output, expected);
//! ```
//!
//! It's also possible to reuse an allocation for `Storage` via the `Into` trait.
//!
//! # Example:
//!
//! ```
//! use sliding_windows::IterExt;
//! use sliding_windows::Storage;
//!
//! let previous_alloca = vec![0u32; 3]; // length doesn't have to be equal to window_size
//! let mut storage: Storage<u32> = Storage::from_vec(previous_alloca, 3);
//! let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];
//!
//! // extra scope so that windowed_iter doesn't outlive storage.into() call
//! {
//!     let windowed_iter = (0..5).sliding_windows(&mut storage);
//!     let output: Vec<Vec<u32>> = windowed_iter.map(|x| x.iter().map(|&x| x).collect()).collect();
//!     assert_eq!(output, expected);
//! }
//!
//! let reusing_alloca: Vec<u32> = storage.into();
//! // keep using allocation of storage
//!
//! ```
//!
//! ### Panics:
//!
//! As this iterator reuses the allocation for the yielded `Window`, no two instances of `Window`
//! belonging to the same iterator may exist simultaneously. As noted above this is checked at runtime.
//!
//! ```
//! use sliding_windows::IterExt;
//! use sliding_windows::Storage;
//!
//! let mut storage: Storage<u32> = Storage::new(3);
//! let mut windowed_iter = (0..5).sliding_windows(&mut storage);
//!
//! // extra scope so that a doesn't live until the for loop
//! {
//!     let a = windowed_iter.next();
//!     //let b = windowed_iter.next(); => this would PANIC
//! }
//!
//! // looping for example is fine though
//! for _ in windowed_iter {
//!     // blah
//! }
//! ```
//!
//! # Mutable Window:
//!
//! There is an implementation of an Iterator over `&'a mut T` for `Window<'a, T>`. It can be obtained
//! by calling `iter_mut()`. For more information see [`Window<'a, T>`](struct.Window.html).
//!
//! However be aware that changes made to the items in the Window are persistent through calls to `next()`.

mod sliding_windows;
#[cfg(test)]
mod tests;

pub use sliding_windows::{
    Storage, Adaptor, Window, WindowIter, WindowIterMut};

pub trait IterExt: Iterator {
    fn sliding_windows(self, storage: &mut Storage<Self::Item>)
        -> Adaptor<Self>
        where Self: Sized
    {
        Adaptor::new(self, storage)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator { }
