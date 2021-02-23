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

use crate::Validator;

#[cfg(feature = "impl_partial_eq_transparent")]
impl<Ctx> PartialEqTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_partial_ord_transparent")]
impl<Ctx> PartialOrdTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_eq_transparent")]
impl<Ctx> EqTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_ord_transparent")]
impl<Ctx> OrdTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_hash_transparent")]
impl<Ctx> HashTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_debug_transparent")]
impl<Ctx> DebugTransparent for Ctx where Ctx: Validator {}

#[cfg(feature = "impl_display_transparent")]
impl<Ctx> DisplayTransparent for Ctx where Ctx: Validator {}
