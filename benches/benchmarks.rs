#![feature(test)]

#[macro_use]
extern crate fc_sort;
extern crate rand;
extern crate test;

use fc_sort::merge_sort;
use rand::random;
use test::Bencher;

macro_rules! sort_n {
    ( $($i:ident => $e:expr),+ ) => {
        $(
        #[bench]
        fn $i(b: &mut Bencher) {
            let list = (0..$e).map(|_| tf!(random::<f64>())).collect::<Vec<_>>();
            b.iter(|| {
                merge_sort(list.clone())
            });
        }
        )+
    }
}

sort_n! {
    sort_100_000 => 100_000,
    sort_200_000 => 200_000,
    sort_300_000 => 300_000,
    sort_400_000 => 400_000,
    sort_500_000 => 500_000,
    sort_600_000 => 600_000,
    sort_700_000 => 700_000,
    sort_800_000 => 800_000,
    sort_900_000 => 900_000
}
