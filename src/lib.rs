#[cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;

mod buf;
mod slice;
pub use buf::StrongBuf;
pub use slice::Strong;

/// Has marker traits for [`Validator`].
pub mod marker;

mod impl_convert;
#[cfg(feature = "diesel")]
pub mod impl_diesel;
#[cfg(feature = "juniper")]
mod impl_juniper;
#[cfg(feature = "serde")]
mod impl_serde;

/// Requires `some_validators` feature.
#[cfg(feature = "some_validators")]
pub mod validators;

/// For [`Strong`]
pub trait Validator {
    type Err;
    #[allow(unused_variables)]
    #[inline]
    fn validate(raw: &str) -> Result<(), Self::Err> { Ok(()) }
}

#[cfg(feature = "shorthand")]
pub use StrongBuf as Str;

#[cfg(feature = "shorthand")]
pub use Strong as S;

pub use std::convert::TryFrom;
