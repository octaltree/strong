/// # Panics
/// Panics if [`Validator`](crate::Validator) rejects blank string.
pub trait DefaultTransparent {}
/// # Panics
/// Panics if [`Validator`](crate::Validator) rejects cloned string.
pub trait CloneTransparent {}

pub trait PartialEqTransparent {}
pub trait EqTransparent {}
pub trait PartialOrdTransparent {}
pub trait OrdTransparent {}
pub trait HashTransparent {}
pub trait DebugTransparent {}
pub trait DisplayTransparent {}
