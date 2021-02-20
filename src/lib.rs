mod buf;
mod slice;
#[cfg(feature = "some_validators")]
mod validators;

pub use buf::Strong;
pub use slice::strong;
#[cfg(feature = "some_validators")]
pub use validators::*;

pub trait Validator {
    type Err;
    #[allow(unused_variables)]
    fn validate(raw: &str) -> Result<(), Self::Err> { Ok(()) }
}
