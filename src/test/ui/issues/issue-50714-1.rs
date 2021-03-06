// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Regression test for issue 50714, make sure that this isn't a linker error.

#![no_std]
#![feature(start)]

extern crate std;

#[start]
fn start(_: isize, _: *const *const u8) -> isize where fn(&()): Eq { //~ ERROR [E0647]
    0
}

