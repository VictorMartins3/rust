//! Using a name through a `#[deprecated]` reexport should warn, issue #30827.

//@ aux-build:deprecated-reexport.rs

#![deny(deprecated)]

extern crate deprecated_reexport as ext;

mod local {
    pub struct Foo;

    #[deprecated(note = "use `Foo` instead")]
    pub use self::Foo as Bar;
}

use local::Bar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
use local::Foo;

use ext::Bar as ExtBar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
use ext::Foo as ExtFoo;

fn main() {
    let _ = Foo;
    let _ = Bar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
    let _ = local::Bar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead

    let _ = ExtFoo;
    let _ = ExtBar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
    let _ = ext::Bar; //~ ERROR use of deprecated re-export `Bar`: use `Foo` instead
    ext::bar(); //~ ERROR use of deprecated re-export `bar`: use `foo` instead
    let _ = ext::Baz; //~ ERROR use of deprecated re-export `Baz`
    let _ = ext::inner::Baz;

    // `old` is deprecated, but the reexport inside it is not.
    let _ = ext::old::Foo;

    #[allow(deprecated)]
    let _ = ext::Bar;
}
