// use std::cmp::Ordering;
// use std::cmp::Ordering::{Equal, Greater, Less};
// use std::collections::BTreeMap;
//
// #[repr(transparent)]
// #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
// pub struct Wavelength(f32);
//
// impl Eq for Wavelength {
//
// }
//
// impl Ord for Wavelength {
//     fn cmp(&self, other: &Self) -> Ordering {
//         f32::exp()
//         if self.0 < other.0 { Less }
//         else if self.0 > other.0 { Greater }
//         else { Equal }
//     }
// }
//
// const STANDARD_OBSERVER_X: BTreeMap<Wavelength, f32> = BTreeMap::from([
//     (Wavelength(0.0), 0.0)
// ]);
