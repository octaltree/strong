use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    StrongBuf, Validator
};
use std::cmp::Ordering;

impl<Ctx: Validator + PartialEqTransparent> PartialEq for StrongBuf<Ctx> {
    #[inline]
    fn eq(&self, other: &StrongBuf<Ctx>) -> bool { self.inner == other.inner }
}

impl<Ctx: Validator + PartialEqTransparent + EqTransparent> Eq for StrongBuf<Ctx> {}

impl<Ctx: Validator + PartialEqTransparent + PartialOrdTransparent> PartialOrd for StrongBuf<Ctx> {
    #[inline]
    fn partial_cmp(&self, other: &StrongBuf<Ctx>) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<
        Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
    > Ord for StrongBuf<Ctx>
{
    #[inline]
    fn cmp(&self, other: &StrongBuf<Ctx>) -> Ordering { self.inner.cmp(&other.inner) }
}
