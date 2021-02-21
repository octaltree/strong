use crate::{
    marker::{DebugTransparent, DisplayTransparent},
    Strong, StrongBuf, Validator
};
use std::fmt;

impl<Ctx> fmt::Debug for Strong<Ctx>
where
    Ctx: Validator + DebugTransparent
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), formatter)
    }
}

impl<Ctx> fmt::Display for Strong<Ctx>
where
    Ctx: Validator + DisplayTransparent
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

impl<Ctx> fmt::Debug for StrongBuf<Ctx>
where
    Ctx: Validator + DebugTransparent
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), formatter)
    }
}

impl<Ctx> fmt::Display for StrongBuf<Ctx>
where
    Ctx: Validator + DisplayTransparent
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}
