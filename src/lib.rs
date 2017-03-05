use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

/// An abstraction over 64 Bit IEEE Floats providing Totality in Ordering and
/// Reflexivity in Equality.
///
/// NaN values are treated as being strictly less than all other values.
/// Including negative infinity. All NaN values are treated as being equal to
/// each other.
#[derive(Copy, Clone)]
pub struct TotalFloat {
    pub inner: f64,
}

// Implement Deref and DerefMut to allow us to use f64 methods on TotalFloat.
//
// Deref and DerefMut allow for coercion to their Target type implicitly when
// passing the source type to a function, or when calling an instance method.
//
// See https://doc.rust-lang.org/book/deref-coercions.html for more.
impl Deref for TotalFloat {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for TotalFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Custom Equality Implementation
impl Eq for TotalFloat {}
impl PartialEq for TotalFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.is_nan() && other.is_nan()) || self.inner.eq(&other.inner)
    }
}

// Custom Ordering Implementation
impl Ord for TotalFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_nan(), other.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => {
                self.inner
                    .partial_cmp(other)
                    .expect("Unexpected Partial Comparison Failure")
            }
        }
    }
}
impl PartialOrd for TotalFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Custom Debug Implementation
// This facilitates printing of TotalFloat in a debug context, as if they were
// f64.
impl fmt::Debug for TotalFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

// Custom Display Implementation
// This facilitates printing of TotalFloat in a display context, as if they were
// f64.
impl fmt::Display for TotalFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// Conversion wrapper from f64 to TotalFloat
impl From<f64> for TotalFloat {
    fn from(from: f64) -> TotalFloat {
        TotalFloat { inner: from }
    }
}

// Conversion wrapper from TotalFloat to f64
impl From<TotalFloat> for f64 {
    fn from(from: TotalFloat) -> f64 {
        from.inner
    }
}

/// Macro for converting f64 to TotalFloat.
#[macro_export]
macro_rules! tf {
    ($float:expr) => { TotalFloat::from($float) }
}

/// Macro for creating lists of TotalFloats.
#[macro_export]
macro_rules! tfvec {
    [$($float:expr),*] => {
        vec![
            $(
                tf!($float)
            ),*
        ]
    }
}

/// Sorts a list of TotalFloat values.
pub fn merge_sort(mut input: Vec<TotalFloat>) -> Vec<TotalFloat> {
    let n = input.len();
    // If there is one element or less of input, we cannot split up the list so
    // it should simply be returned. Otherwise, recursively call merge_sort on
    // the left and right half of the list.
    if n <= 1 {
        input
    } else {
        merge(// Takes half of the input (removing it) and merge_sorts it
              merge_sort(input.split_off(n / 2)),
              // Takes the remaning half of the input and merge_sorts it
              merge_sort(input))
    }
}

/// Merges two lists of TotalFloat values into an ordered list of TotalFloat
/// values.
pub fn merge(mut a: Vec<TotalFloat>,
             mut b: Vec<TotalFloat>)
             -> Vec<TotalFloat> {
    // Declare a new buffer to be our returning data.
    // Size it such that it will not reallocate.
    let mut buffer = Vec::with_capacity(a.len() + b.len());

    // Create drains of `a` and `b`, these are iterators that remove elements as
    // they are returned.
    let mut a = a.drain(..);
    let mut b = b.drain(..);

    // Access the first elements of a and b
    let mut next_a = a.next();
    let mut next_b = b.next();

    // Repeat until internal break condition met, which will be when a and b are
    // both empty.
    loop {
        match (next_a, next_b) {
            // If a and b are both not empty
            (Some(at), Some(bt)) => {
                // Push the lesser element to the buffer, and advance it's
                // iterator.
                if at > bt {
                    buffer.push(bt);
                    next_b = b.next();
                } else {
                    buffer.push(at);
                    next_a = a.next();
                }
            }
            // If a is not empty, and b is
            (Some(at), None) => {
                // Push the yielded element of a to the buffer, and advance a's
                // iterator.
                buffer.push(at);
                next_a = a.next();
            }
            // If b is not empty, and a is
            (None, Some(bt)) => {
                // Push the yielded element of b to the buffer, and advance b's
                // iterator.
                buffer.push(bt);
                next_b = b.next();
            }
            // If both a and b are empty, exit the loop
            (None, None) => break
        }
    }

    // Return the buffer
    buffer
}
