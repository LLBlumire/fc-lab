use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

/// An abstraction over 64 Bit IEEE Floats providing Totality in Ordering and Reflexivity in
/// Equality.
/// 
/// NaN values are treated as being strictly less than all other values. Including negative
/// infinity. All NaN values are treated as being equal to each other.
#[derive(Copy, Clone)]
pub struct TotalFloat {
    pub inner: f64,
}

// Implement Deref and DerefMut to allow us to use f64 methods on TotalFloat.
impl Deref for TotalFloat {
    type Target = f64;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl DerefMut for TotalFloat {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

// Custom Equality Implementation
impl Eq for TotalFloat { }
impl PartialEq for TotalFloat {
    fn eq(&self, other: &Self) ->  bool {
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
            (false, false) => self.partial_cmp(other).expect("Unexpected Partial Comparison Failure")
        }
    }
}
impl PartialOrd for TotalFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Custom Debug Implementation
impl fmt::Debug for TotalFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

// Custom Display Implementation
impl fmt::Display for TotalFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Macro for converting to Transreal floats.
#[macro_export]
macro_rules! tf {
    ($float:expr) => {
        $crate::TotalFloat {
            inner: $float,
        }
    }
}

// Conversion wrapper from f64 to TotalFloat
impl From<f64> for TotalFloat {
    fn from(from: f64) -> TotalFloat { tf!(from) }
}

// Conversion wrapper from TotalFloat to f64
impl From<TotalFloat> for f64 {
    fn from(from: TotalFloat) -> f64 { from.inner }
}

/// Macro for creating lists of Transreal floats.
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

fn main() {
    let my_list = tfvec![1.0, 2.0, 5.0, 3.0, 192.0, (-0.0/0.0)];
    println!("{:?}", my_list);
}
