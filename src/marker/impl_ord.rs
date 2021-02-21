use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    Strong, StrongBuf, Validator
};
use std::cmp::Ordering;

impl<Ctx: Validator + PartialEqTransparent> PartialEq for Strong<Ctx> {
    #[inline]
    fn eq(&self, other: &Strong<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx: Validator + PartialEqTransparent + EqTransparent> Eq for Strong<Ctx> {}

impl<Ctx: Validator + PartialEqTransparent + PartialOrdTransparent> PartialOrd for Strong<Ctx> {
    #[inline]
    fn partial_cmp(&self, other: &Strong<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<
        Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
    > Ord for Strong<Ctx>
{
    #[inline]
    fn cmp(&self, other: &Strong<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}

impl<Ctx: Validator + PartialEqTransparent> PartialEq for StrongBuf<Ctx> {
    #[inline]
    fn eq(&self, other: &StrongBuf<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx: Validator + PartialEqTransparent + EqTransparent> Eq for StrongBuf<Ctx> {}

impl<Ctx: Validator + PartialEqTransparent + PartialOrdTransparent> PartialOrd for StrongBuf<Ctx> {
    #[inline]
    fn partial_cmp(&self, other: &StrongBuf<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<
        Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
    > Ord for StrongBuf<Ctx>
{
    #[inline]
    fn cmp(&self, other: &StrongBuf<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}