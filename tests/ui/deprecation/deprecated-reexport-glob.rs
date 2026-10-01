//! Names brought in through a glob still warn if they come from a deprecated reexport.

//@ aux-build:deprecated-reexport.rs

#![deny(deprecated)]

extern crate deprecated_reexport;

use deprecated_reexport::*;

fn main() {
    let _ = Foo;
    let _ = Bar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
}
