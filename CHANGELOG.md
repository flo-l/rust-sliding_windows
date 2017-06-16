CHANGELOG
=================

### 3.0
This release breaks the old API in favour of huge performance gains. Before 3.0 it was
possible to deref a Window into a slice of T. This made it necessary to place the
elements of the wrapped Iterator sequentially in memory. So one had to either allocate
a lot of memory or copy a lot.

This is no longer the case. The new API includes 2 Iterators (over &T and &mut T)
for Window. This makes it possible to avoid copying **ENTIRELY**.

- Remove Storage::new_exact and Storage::optimized, they are no longer needed
- Remove impl of Deref/DerefMut for Window
- Add WindowIter/WindowIterMut instead
- Add CHANGELOG.md
