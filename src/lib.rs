// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Passive
//!
//! See the documentation of the three marker traits defined by this crate for
//! more information.

#![no_std]

use core::cell::{Cell, UnsafeCell};
use core::mem::{ManuallyDrop, MaybeUninit};

/// Types whose values are always aligned, i.e. with an alignment of 1.
pub unsafe trait AlwaysAligned {}

/// Types for which all possible bit patterns represent a valid value.
///
/// Note that `MaybeUninit<T>` and `[T; 0]` both implement this marker trait
/// even if `T` does not.
pub unsafe trait AlwaysValid {}

/// Types whose values are devoid of inner mutability.
///
/// Note that `[T; 0]` implements this marker trait even if `T` does not.
pub unsafe trait Immutable {}

macro_rules! implements {
    ($($ty:tt $($qualifier:ident)+,)+) => {$(
        $(qualifier! { $qualifier $ty })+
    )+};
}

macro_rules! qualifier {
    (a [$({$($param:ident),*})? $ty:ty]) => {
        unsafe impl<$($($param),*)?> AlwaysAligned for $ty {}
    };
    (v [$({$($param:ident),*})? $ty:ty]) => {
        unsafe impl<$($($param),*)?> AlwaysValid for $ty {}
    };
    (i [$({$($param:ident),*})? $ty:ty]) => {
        unsafe impl<$($($param),*)?> Immutable for $ty {}
    };
}

// `a` = `AlwaysAligned`
// `v` = `AlwaysValid`
// `i` = `Immutable`

implements! {
    [()]                      a v i,
    [u8]                      a v i,
    [i8]                      a v i,

    [bool]                    a   i,

    [u16]                       v i,
    [u32]                       v i,
    [u64]                       v i,
    [u128]                      v i,
    [usize]                     v i,

    [i16]                       v i,
    [i32]                       v i,
    [i64]                       v i,
    [i128]                      v i,
    [isize]                     v i,

    [{T} ManuallyDrop<T>]     a v i,
    [{T} MaybeUninit<T>]      a   i,
    [{T} Cell<T>]             a v  ,
    [{T} UnsafeCell<T>]       a v  ,
}

unsafe impl<T> AlwaysValid for MaybeUninit<T> {}

macro_rules! tuple_implements {
    ($(($($param:ident),+),)+) => {
        implements! {
            $([{$($param),+} ($($param,)+)] a v i,)+
        }
    };
}

tuple_implements! {
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
}

macro_rules! array_implements {
    ($($len:expr,)+) => {
        implements! {
            $([{T} [T; $len]] a v i,)+
        }
    };
}

array_implements! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
}

unsafe impl<T> AlwaysAligned for [T; 0] where T: AlwaysAligned {}
unsafe impl<T> AlwaysValid for [T; 0] {}
unsafe impl<T> Immutable for [T; 0] {}
