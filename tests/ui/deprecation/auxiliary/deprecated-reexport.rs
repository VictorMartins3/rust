pub struct Foo;

pub fn foo() {}

#[deprecated(since = "1.0.0", note = "use `Foo` instead")]
pub use self::Foo as Bar;

#[deprecated(note = "use `foo` instead")]
pub use self::foo as bar;

pub mod inner {
    pub struct Baz;
}

#[deprecated]
pub use self::inner::Baz;

#[deprecated(note = "deprecated module")]
pub mod old {
    // Reexports don't inherit the deprecation of their parent module.
    pub use crate::Foo;
}
