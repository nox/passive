// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

use core::cell::Cell;
use core::mem::ManuallyDrop;
use passive::{AlwaysAligned, AlwaysValid, Immutable};

#[derive(AlwaysAligned)]
struct _TestAlwaysAligned {
    unit: (),
    byte: u8,
    pair: (u8, bool),
    array: [u8; 2],
    cell: Cell<u8>,
}
#[derive(AlwaysValid)]
struct _TestAlwaysValid {
    byte: i8,
    pair: (u8, u16),
    array: [u32; 2],
    cell: Cell<usize>,
}

#[derive(Immutable)]
struct _TestImmutable {
    bool: bool,
    byte: i8,
    pair: (u8, u16),
    array: [u32; 2],
}

#[derive(AlwaysAligned, AlwaysValid, Immutable)]
struct _TestGeneric<T> {
    manually_drop: ManuallyDrop<T>,
}

#[test]
fn it_works() {
    // The things that this test do will fail to compile if the code is wrong,
    // this test exists only so that `cargo test` doesn't say that 0 tests
    // passed.
}
