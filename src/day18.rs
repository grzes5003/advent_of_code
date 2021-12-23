// use std::ops::Add;
//
// trait Unit<T> : Add {}
//
// trait Pair<T> {}
// trait Single<T> {}
//
// impl<T> Unit<T> for dyn Pair<T> {
//
// }
//
// impl<T> Unit<T> for dyn Single<T> {
//
// }
//
// struct Exp<T> {
//     _1: Box<dyn Unit<T>>,
//     _2: Box<dyn Unit<T>>
// }