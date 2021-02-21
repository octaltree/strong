mod buf;
mod slice;
pub use buf::StrongBuf;
pub use slice::Strong;

#[cfg(feature = "some_validators")]
mod validators;
#[cfg(feature = "some_validators")]
pub use validators::*;

pub trait Validator {
    type Err: std::fmt::Debug;
    #[allow(unused_variables)]
    fn validate(raw: &str) -> Result<(), Self::Err> { Ok(()) }
}
