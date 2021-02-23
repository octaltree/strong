pub trait PartialEqTransparent {}
pub trait EqTransparent {}
pub trait PartialOrdTransparent {}
pub trait OrdTransparent {}
pub trait HashTransparent {}
pub trait DebugTransparent {}
pub trait DisplayTransparent {}

mod impl_fmt;
mod impl_hash;
mod impl_ord;
