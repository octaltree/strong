use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    Strong, StrongBuf, Validator
};
use std::cmp::Ordering;

impl<Ctx> PartialEq for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent
{
    #[inline]
    fn eq(&self, other: &Strong<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx> Eq for Strong<Ctx> where Ctx: Validator + PartialEqTransparent + EqTransparent {}

impl<Ctx> PartialOrd for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
{
    #[inline]
    fn partial_cmp(&self, other: &Strong<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<Ctx> Ord for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
{
    #[inline]
    fn cmp(&self, other: &Strong<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}

impl<Ctx> PartialEq for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent
{
    #[inline]
    fn eq(&self, other: &StrongBuf<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx> Eq for StrongBuf<Ctx> where Ctx: Validator + PartialEqTransparent + EqTransparent {}

impl<Ctx> PartialOrd for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
{
    #[inline]
    fn partial_cmp(&self, other: &StrongBuf<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<Ctx> Ord for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
{
    #[inline]
    fn cmp(&self, other: &StrongBuf<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}
