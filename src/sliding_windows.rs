use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};
use std::fmt;

/// This holds the backing allocation for the `Window` of a `Adaptor`.
///
/// See [sliding_windows](index.html) for more information.
pub struct Storage<T> {
    window_size: usize,
    // use elements just from this offset on
    window_offset: Cell<usize>,
    /// acts as a refcount
    uniquely_owned: Cell<bool>,
    data: UnsafeCell<Vec<T>>
}

impl<T> Storage<T> {
    /// Create a new `Storage` with a given window size optimized for a given Iterator.
    /// This will allocate an amount of memory big enough to avoid shifting elements during iteration.
    ///
    /// This should be the most performant option, which uses the most memory: ```Iterator::size_hint() * mem::size_of<Iterator::Item>```.
    /// If there is no size_hint, this is identical to `Storage::new()`.
    ///
    /// If you want to use less memory, but more CPU, consider using ```Storage::new()``` or ```Storage::new_exact()``` instead.
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn optimized<I: Iterator>(iter: &I, window_size: usize) -> Storage<T> {
        let size = iter.size_hint();
        let mem = size.1.unwrap_or(size.0);

        if mem == 0 {
            Storage::new(window_size)
        } else if mem < window_size {
            Storage::new_exact(mem)
        } else {
            Storage::from_vec(Vec::with_capacity(mem), window_size)
        }
    }

    /// Create a new `Storage` with a given window size.
    /// This will allocate twice as much memory as is needed to store the Window for performance reasons.
    ///
    /// If you want to use as few memory as possible, but more CPU, consider using ```Storage::new_exact()``` instead.
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn new(window_size: usize) -> Storage<T> {
        Storage::from_vec(Vec::with_capacity(window_size * 2), window_size)
    }

    /// Create a new `Storage` with a given window size.
    /// This will allocate exactly as much memory as is needed to store the Window.
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn new_exact(window_size: usize) -> Storage<T> {
        Storage::from_vec(Vec::with_capacity(window_size), window_size)
    }

    /// Create a new `Storage` with a given window size from a given struct implementing `Into<Vec>`.
    /// The contents of the Vec will be removed.
    /// This will reuse the allocation of the Vec instead of allocating new memory.
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn from_vec<S: Into<Vec<T>>>(vec: S, window_size: usize) -> Storage<T> {
        Storage {
            window_size: window_size,
            window_offset: Cell::new(0),
            uniquely_owned: Cell::new(true),
            data: UnsafeCell::new(vec.into())
        }
    }

    fn new_window<'a>(&'a self) -> Window<'a, T> {
        // assert that the last window went out of scope
        assert!(self.uniquely_owned.get(), "next() called before previous Window went out of scope");
        let data = unsafe { &mut *self.data.get() };
        let window_offset = self.window_offset.get();

        self.uniquely_owned.set(false);

        Window { drop_flag: &self.uniquely_owned, data: &mut data[window_offset..] }
    }

    // push value onto self
    // this should only be called if data.len() >= window_size
    fn push(&self, elt: T) -> bool {
        assert!(self.uniquely_owned.get(), "next() called before previous Window went out of scope");
        let data = unsafe { &mut *self.data.get() };
        let window_offset = self.window_offset.get();

        if data.len() >= self.window_size {
            // storage is full, copy elements to the front with drain
            if data.len() == data.capacity() {
                data.drain(0..window_offset+1);
                self.window_offset.set(0);
            // storage has min. window_size elements, so offset must be increased
            } else {
                self.window_offset.set(window_offset + 1);
            }
        }

        data.push(elt);
        data.len() >= self.window_size
    }

    // clear backing storage
    fn clear(&self) {
        assert!(self.uniquely_owned.get(), "next() called before previous Window went out of scope");
        let data = unsafe { &mut *self.data.get() };
        data.clear();
    }
}

impl<T> Into<Vec<T>> for Storage<T> {
    fn into(self) -> Vec<T> {
        assert!(self.uniquely_owned.get(), "Storage dereferenced before previous Window went out of scope");
        unsafe {
            self.data.into_inner()
        }
    }
}

/// This is the `Item` type of the `Adaptor` iterator.
///
/// # Usage:
///
/// `Window<'a, T>` dereferences to `&'a [T]` or `&'a mut [T]`.
///
/// ```
/// use sliding_windows::IterExt;
/// use sliding_windows::Storage;
///
/// let mut storage: Storage<u32> = Storage::new(3);
/// let mut windowed_iter = (0..5).sliding_windows(&mut storage);
///
/// for mut window in windowed_iter {
///     // extra scope, so that later mutable borrow is possible
///     {
///         let slice: &[u32] = &window;
///         // work with slice
///     }
///
///     // mutable
///     let mut_slice: &mut [u32] = &mut window;
///     // work with data mutably
/// }
/// ```
///
/// See [sliding_windows](index.html) for more information.
pub struct Window<'a, T: 'a> {
    drop_flag: &'a Cell<bool>,
    data: &'a mut [T],
}

impl<'a, T> fmt::Debug for Window<'a, T> where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<'a, T> Drop for Window<'a, T> {
    fn drop(&mut self) {
        // set flag to indicate this window was dropped
        self.drop_flag.set(true);
    }
}

// convenience impl &Window<T> => &[T]
impl<'a, T> Deref for Window<'a, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        debug_assert!(!self.drop_flag.get());
        &*self.data
    }
}

// convenience impl &mut Window<T> => &mut [T]
impl<'a, T> DerefMut for Window<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        debug_assert!(!self.drop_flag.get());
        self.data
    }
}

impl<'a, 'b, T> PartialEq<&'b [T]> for Window<'a, T> where T: PartialEq
{
    fn eq(&self, other: &&'b [T]) -> bool {
        &self[..] == *other
    }
}

/// See [sliding_windows](index.html) for more information.
pub struct Adaptor<'a, I: Iterator> where <I as Iterator>::Item: 'a {
    iter: I,
    done: bool,
    storage: &'a Storage<I::Item>,
}

impl<'a, I: Iterator> Adaptor<'a, I> {
    /// This creates a new Adaptor. Usually you should be using
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn new(iter: I, storage: &'a Storage<I::Item>) -> Adaptor<'a, I> {
        // in case the storage was reused
        storage.clear();

        Adaptor {
            iter: iter,
            done: false,
            storage: storage,
        }
    }
}

impl<'a, I: Iterator> Iterator for Adaptor<'a, I> {
    type Item = Window<'a, I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.storage.window_size == 0 {
            return None
        }
        self.done = true;
        for elt in &mut self.iter {
            if self.storage.push(elt) {
                self.done = false;
                break;
            }
        }
        if !self.done {
            // return new window
            Some(self.storage.new_window())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.storage.window_size;
        let (mut lower, mut upper): (usize, Option<usize>) = self.iter.size_hint();

        if size == 0 {
            return (0, None);
        }

        lower = match lower {
            0 => 0,
            x if x >= size => x - size + 1,
            _ => 1
        };

        upper = upper.map(|upper|
            match upper {
                0 => 0,
                x if x >= size => x - size + 1,
                _ => 1
            }
        );

        (lower, upper)
    }
}
