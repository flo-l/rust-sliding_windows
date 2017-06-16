use std::cell::{Cell, UnsafeCell};
use std::fmt;
use std::marker::PhantomData;

/// This holds the backing allocation for the `Window` of a `Adaptor`.
///
/// See [sliding_windows](index.html) for more information.
pub struct Storage<T> {
    window_size: usize,
    // this is the offset of the first element
    window_offset: Cell<usize>,
    /// acts as a refcount
    uniquely_owned: Cell<bool>,
    data: UnsafeCell<Vec<T>>,
}

impl<T> Storage<T> {
    /// Create a new `Storage` with a given window size.
    /// This will allocate twice as much memory as is needed to store the Window for performance reasons.
    ///
    /// If you want to use as few memory as possible, but more CPU, consider using ```Storage::new_exact()``` instead.
    ///
    /// See [sliding_windows](index.html) for more information.
    pub fn new(window_size: usize) -> Storage<T> {
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

        Window { drop_flag: &self.uniquely_owned, data: &mut data[..], window_offset: window_offset }
    }

    // push value onto self, return true if window is full (for initialization)
    // this assumes that data.capacity >= self.window_size
    fn push(&self, elt: T) -> bool {
        assert!(self.uniquely_owned.get(), "next() called before previous Window went out of scope");
        let data = unsafe { &mut *self.data.get() };
        let window_offset = self.window_offset.get();

        // if storage is not full simply push the element
        // this is only the case when filling storage initially
        if data.len() < self.window_size
        {
            data.push(elt);
            return data.len() == self.window_size;
        }

        debug_assert!(data.len() == self.window_size);

        // the storage is full, overwrite the last element
        let new_offset;
        if window_offset >= (self.window_size - 1) {
            new_offset = 0;
        } else {
            new_offset = window_offset + 1;
        }

        data[window_offset] = elt;
        self.window_offset.set(new_offset);
        true
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
/// `&Window<'a, T>` implements `into_iter()`, which returns an Iterator over `&T`.
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
///         for x in &window {
///             // work with data immutably
///         }
///     }
///
///     // mutable
///     let mut iter_mut = window.iter_mut();
///     for x in iter_mut {
///         // work with data mutably (affecting the next windows of course)
///     }
/// }
/// ```
///
/// See [sliding_windows](index.html) for more information.
pub struct Window<'a, T: 'a> {
    drop_flag: &'a Cell<bool>,
    // index of first element
    window_offset: usize,
    data: &'a mut [T],
}

impl<'a, T> Window<'a, T>
{
    pub fn iter(&self) -> WindowIter<T> {
        WindowIter {
            data: self.data,
            current_index: self.window_offset,
            iteration_num: 0
        }
    }

    pub fn iter_mut(&mut self) -> WindowIterMut<T> {
        WindowIterMut {
            data: self.data.as_mut_ptr(),
            data_len: self.data.len(),
            current_index: self.window_offset,
            iteration_num: 0,
            _p: PhantomData
        }
    }
}

impl<'a, T> fmt::Debug for Window<'a, T> where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Window")?;
        return f.debug_list().entries(self.into_iter()).finish();
    }
}

impl<'a, T> Drop for Window<'a, T> {
    fn drop(&mut self) {
        // set flag to indicate this window was dropped
        self.drop_flag.set(true);
    }
}

impl<'a, 'b, T> PartialEq<&'b [T]> for Window<'a, T> where T: PartialEq
{
    fn eq(&self, other: &&'b [T]) -> bool {
        if self.data.len() != other.len() { return false }
        for (i, x) in self.into_iter().enumerate() {
            if *x != other[i] { return false }
        }
        true
    }
}

impl<'a, T> IntoIterator for &'a Window<'a, T>
{
    type Item = &'a T;
    type IntoIter = WindowIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct WindowIter<'a, T: 'a>
{
    data: &'a [T],
    current_index: usize,
    // number of next() calls made which returned Some(_)
    iteration_num: usize,
}

impl<'a, T> Iterator for WindowIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_element = &self.data[self.current_index];

        if self.iteration_num >= self.data.len() {
            // the end was reached
            return None;
        } else if self.current_index >= (self.data.len() - 1) {
            // wrap around if the increment would create an invalid index
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.iteration_num += 1;
        Some(current_element)
    }
}

pub struct WindowIterMut<'a, T: 'a>
{
    data: *mut T,
    data_len: usize,
    current_index: usize,
    // number of next() calls made which returned Some(_)
    iteration_num: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T> Iterator for WindowIterMut<'a, T>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_element = unsafe { self.data.offset(self.current_index as isize).as_mut().unwrap() };

        if self.iteration_num >= self.data_len {
            // the end was reached
            return None;
        } else if self.current_index >= (self.data_len - 1) {
            // wrap around if the increment would create an invalid index
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.iteration_num += 1;
        Some(current_element)
    }
}

// TODO add ExactSizeIterator
// TODO add other stuff like DoubleEndedIterator etc.

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
            return None;
        }
        self.done = true;

        for elt in &mut self.iter {
            self.done = false;
            if self.storage.push(elt) {
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
