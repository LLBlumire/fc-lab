#[macro_use]
extern crate fc_sort;
extern crate rand;

use fc_sort::merge_sort;
use rand::random;

use std::f64::INFINITY as INF;
use std::f64::NEG_INFINITY as N_INF;
use std::f64::NAN;

#[test]
fn empty_list() {
    // Sort an empty lists
    assert_eq!(merge_sort(tfvec![]), tfvec![]);
}

#[test]
fn one_element_list() {
    // Sort a single element list
    assert_eq!(merge_sort(tfvec![1.0]), tfvec![1.0]);
}

#[test]
fn two_element_list() {
    // Sort a two element list
    assert_eq!(merge_sort(tfvec![2.0, 1.0]), tfvec![1.0, 2.0]);
}

#[test]
fn general_element_list() {
    // Sort a list with an arbitrary number of elemets (here 6)
    assert_eq!(merge_sort(tfvec![3.0, 2.0, 5.3, 6.1, 8.4, 1.0]),
               tfvec![1.0, 2.0, 3.0, 5.3, 6.1, 8.4]);
}

#[test]
fn some_elements_same() {
    // Sort a list where some of the elements are the same (here 3.4)
    assert_eq!(merge_sort(tfvec![3.4, 1.2, 8.4, 3.4, 2.3]),
               tfvec![1.2, 2.3, 3.4, 3.4, 8.4])
}

#[test]
fn some_elements_infinity() {
    // Sort a list where some elements are infinity or negative infinity
    assert_eq!(merge_sort(tfvec![INF, 4.4, 2.1, N_INF, 5.2, INF]),
               tfvec![N_INF, 2.1, 4.4, 5.2, INF, INF]);
}

#[test]
fn some_elements_minus_zero() {
    // Sort a list where some elements are negative zero
    assert_eq!(merge_sort(tfvec![2.3, -9.0, -0.0, 4.2, 0.0, 9.1]),
               tfvec![-9.0, -0.0, 0.0, 2.3, 4.2, 9.1]);
}

#[test]
fn some_elements_nan() {
    // Sort a list where some elements are NAN
    assert_eq!(merge_sort(tfvec![3.4, 1.2, 8.4, NAN, 2.3, N_INF]),
               tfvec![NAN, N_INF, 1.2, 2.3, 3.4, 8.4]);
}

#[test]
fn pseudo_random() {
    // Generate a random 100 element list (highly unlikely to be presorted).
    let list = (0..100).map(|_| tf!(random::<f64>())).collect();

    // Sort the list (moves out data, so we shadow the old variable name).
    let list = merge_sort(list);

    // Check the list is sorted by comparing each element to the following
    // element, and checking that they all compare lesser or equal. This assumes
    // the transitive property holds.
    assert!(list.iter().zip(list.iter().skip(1)).all(|(a, b)| a <= b))
}

#[test]
fn nan_with_different_mantissa() {
    // The program was specified to treat NaN == NaN, therefore all NaN values
    // will compare the same regardless of mantissa.

    // Construct both as integers from a bitpattern For reference, a standard
    // (0.0 / 0.0) NaN would be 0x7ff8000000000000.
    let nan1: u64 = 0x7ff800a004001000;
    let nan2: u64 = 0x7ff80090e200a000;

    // Perform a pointer cast to turn both into floats
    let nan1: f64 = unsafe { *(&nan1 as *const u64 as *const f64) };
    let nan2: f64 = unsafe { *(&nan2 as *const u64 as *const f64) };

    // Verify that as TotalFloats the NaNs compare equal
    assert_eq!(tf!(nan1), tf!(nan2));

    // Demonstrate usage in sort, NAN is used in comparison because as
    // demonstrated above, both will compare equal to any NAN value, so the
    // comparator doesn't matter).
    assert_eq!(merge_sort(tfvec![2.0, -4.2, INF, 2.1, nan1, 3.2, nan2]),
               tfvec![NAN, NAN, -4.2, 2.0, 2.1, 3.2, INF]);
}
