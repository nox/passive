// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Packed integers that are [always aligned](trait.AlwaysAligned.html),
//! [always valid](trait.AlwaysValid.html), and obviously
//! [immutable](trait.Immutable.html).

use core::mem;

macro_rules! item_with_computed_doc {
    (#[doc = $doc:expr] $item:item) => { #[doc = $doc] $item };
}

macro_rules! inline_fn_with_computed_doc {
    ($(#[doc = $doc:expr])+ #[inline] $($tt:tt)+) => {
        $(#[doc = $doc])+ #[inline] $($tt)+
    };
}

macro_rules! packed_integers {
    ($($packed:ident: $ty:ident,)+) => {$(
        item_with_computed_doc! {
            #[doc = concat!("A packed `", stringify!($ty), "`.")]
            #[repr(transparent)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub struct $packed {
                bytes: [u8; mem::size_of::<$ty>()],
            }
        }

        unsafe impl crate::AlwaysAligned for $packed {}
        unsafe impl crate::AlwaysValid for $packed {}
        unsafe impl crate::Immutable for $packed {}

        impl $packed {
            inline_fn_with_computed_doc! {
                #[doc = concat!(
                    "Creates a packed `", stringify!($ty), "` value from its ",
                    "raw bytes.")]
                #[inline]
                pub fn from_bytes(bytes: [u8; mem::size_of::<$ty>()]) -> Self {
                    Self { bytes }
                }
            }

            inline_fn_with_computed_doc! {
                #[doc = concat!(
                    "Creates an `", stringify!($ty), "` from this ",
                    "packed integer, reading its bytes in little endian.")]
                #[inline]
                pub fn to_le(self) -> $ty {
                    $ty::from_le_bytes(self.bytes)
                }
            }

            inline_fn_with_computed_doc! {
                #[doc = concat!(
                    "Creates an `", stringify!($ty), "` from this ",
                    "packed integer, reading its bytes in big endian.")]
                #[inline]
                pub fn to_be(self) -> $ty {
                    $ty::from_be_bytes(self.bytes)
                }
            }

            inline_fn_with_computed_doc! {
                #[doc = concat!(
                    "Creates an `", stringify!($ty), "` from this ",
                    "packed integer, reading its bytes in native endian.")]
                #[inline]
                pub fn to_ne(self) -> $ty {
                    $ty::from_ne_bytes(self.bytes)
                }
            }
        }
    )+}
}

packed_integers! {
    PackedU16: u16,
    PackedU32: u32,
    PackedU64: u64,
    PackedU128: u128,
    PackedUsize: usize,

    PackedI16: i16,
    PackedI32: i32,
    PackedI64: i64,
    PackedI128: i128,
    PackedIsize: isize,
}
