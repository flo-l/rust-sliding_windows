mod sliding_windows;
#[cfg(test)]
mod tests;

pub use sliding_windows::{
    SlidingWindowStorage, SlidingWindowAdaptor };

pub trait IterExt: Iterator {
    /// Return an iterator adaptor that yields sliding windows into the elements of the wrapped Iterator.
    ///
    /// Note that this adaptor does **NOT** clone the whole window on every call to `next()`.
    ///
    /// Iterator element type is `Window<'a, Self::Item>`.
    ///
    /// Note: `Window<'a, Self::Item>` dereferences to `&'a [Self::Item]` or `&'a mut [Self::Item]`
    ///
    /// This iterator is *fused*.
    ///
    /// # Example:
    ///
    /// ```
    /// use sliding_windows::IterExt;
    /// use sliding_windows::SlidingWindowStorage;
    ///
    /// let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);
    /// let windowed_iter = (0..5).sliding_windows(&mut storage);
    /// let output: Vec<Vec<u32>> = windowed_iter.map(|x| From::from(&x[..])).collect();
    /// let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];
    ///
    /// assert_eq!(output, expected);
    /// ```
    ///
    /// It's also possible to reuse an allocation for `SlidingWindowStorage` via the `Into` trait.
    ///
    /// # Example:
    ///
    /// ```
    /// use sliding_windows::IterExt;
    /// use sliding_windows::SlidingWindowStorage;
    ///
    /// let previous_alloca = vec![0u32; 3]; // length doesn't have to be equal to window_size
    /// let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::from_vec(previous_alloca, 3);
    /// let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];
    ///
    /// // extra scope so that windowed_iter doesn't outlive storage.into() call
    /// {
    ///     let windowed_iter = (0..5).sliding_windows(&mut storage);
    ///     let output: Vec<Vec<u32>> = windowed_iter.map(|x| From::from(&x[..])).collect();
    ///     assert_eq!(output, expected);
    /// }
    ///
    /// let reusing_alloca: Vec<u32> = storage.into();
    /// // keep using allocation of storage
    /// 
    /// ```
    ///
    /// ### Panics:
    ///
    /// As this iterator reuses the allocation for the yielded `Window`, no two instances of `Window`
    /// belonging to the same iterator may exist simultaneously. This is checked at runtime.
    ///
    /// ```
    /// use sliding_windows::IterExt;
    /// use sliding_windows::SlidingWindowStorage;
    ///
    /// let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);
    /// let mut windowed_iter = (0..5).sliding_windows(&mut storage);
    ///
    /// // extra scope so that a doesn't live until the for loop
    /// {
    ///     let a = windowed_iter.next();
    ///     //let b = windowed_iter.next(); => this would PANIC
    /// }
    /// 
    /// // looping for example is fine though
    /// for _ in windowed_iter {
    ///     // blah
    /// }
    /// ```
    /// 
    /// # Mutable Window:
    /// Window does not only dereference to an immutable slice of `Self::Item`, it also dereferences
    /// to a mutable slice of `Self::Item`. Items of the mutable slice may be mutated freely.
    /// 
    /// However be aware that changes made to the items in the Window are persistent through calls to `next()`.
    fn sliding_windows(self, storage: &mut SlidingWindowStorage<Self::Item>)
        -> SlidingWindowAdaptor<Self>
        where Self: Sized
    {
        SlidingWindowAdaptor::new(self, storage)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator { }
