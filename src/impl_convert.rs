//! impl From, TryFrom
use crate::{Strong, StrongBuf, Validator};
use std::{convert::TryFrom, str::FromStr};

impl<Ctx> TryFrom<String> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    type Error = Ctx::Err;
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::validate(s) }
}

impl<'a, Ctx> TryFrom<&'a str> for &'a Strong<Ctx>
where
    Ctx: Validator
{
    type Error = Ctx::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> { Strong::validate(s) }
}

impl<Ctx> FromStr for StrongBuf<Ctx>
where
    Ctx: Validator
{
    type Err = Ctx::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::validate(s.to_string()) }
}

impl<Ctx> AsRef<Strong<Ctx>> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    fn as_ref(&self) -> &Strong<Ctx> { self }
}

// TODO: Box, Rc, Arc, Cow
