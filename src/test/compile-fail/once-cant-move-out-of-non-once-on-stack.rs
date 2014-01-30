// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Testing guarantees provided by once functions.
// This program would segfault if it were legal.

extern mod sync;
use sync::Arc;

fn foo(blk: ||) {
    blk();
    blk();
}

fn main() {
    let x = Arc::new(true);
    foo(|| {
        assert!(*x.get());
        drop(x); //~ ERROR cannot move out of captured outer variable
    })
}
