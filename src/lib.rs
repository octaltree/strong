mod buf;
mod slice;
pub use buf::StrongBuf;
pub use slice::Strong;

/// Has marker traits for [`Validator`].
pub mod marker;

mod impl_convert;

/// Requires `some_validators` feature.
#[cfg(feature = "some_validators")]
pub mod validators;

/// For [`Strong`]
pub trait Validator {
    type Err: std::fmt::Debug;
    #[allow(unused_variables)]
    #[inline]
    fn validate(raw: &str) -> Result<(), Self::Err> { Ok(()) }

    // fn try_from<S, T>(raw: S) -> Result<T, Self::Err>
    // where
    //    T: TryFrom<S>
    //{
    //    TryFrom::try_from(raw)
    //}
}

#[cfg(feature = "shorthand")]
pub use StrongBuf as Str;

#[cfg(feature = "shorthand")]
pub use Strong as S;

pub use std::convert::TryFrom;
