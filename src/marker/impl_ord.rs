use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    Strong, StrongBuf, Validator
};
use std::{borrow::Cow, cmp, cmp::Ordering};

impl<Ctx> PartialEq for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent
{
    #[inline]
    fn eq(&self, other: &Strong<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx> PartialEq for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent
{
    #[inline]
    fn eq(&self, other: &StrongBuf<Ctx>) -> bool { <Strong<Ctx> as PartialEq>::eq(self, other) }
}

impl<Ctx> PartialOrd for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
{
    #[inline]
    fn partial_cmp(&self, other: &Strong<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<Ctx> PartialOrd for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
{
    #[inline]
    fn partial_cmp(&self, other: &StrongBuf<Ctx>) -> Option<Ordering> {
        <Strong<Ctx> as PartialOrd>::partial_cmp(self, other)
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b, Ctx> PartialEq<$rhs> for $lhs
        where
            Ctx: Validator + PartialEqTransparent
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { <Strong<Ctx> as PartialEq>::eq(self, other) }
        }

        impl<'a, 'b, Ctx> PartialEq<$lhs> for $rhs
        where
            Ctx: Validator + PartialEqTransparent
        {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { <Strong<Ctx> as PartialEq>::eq(self, other) }
        }

        impl<'a, 'b, Ctx> PartialOrd<$rhs> for $lhs
        where
            Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
        {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<cmp::Ordering> {
                <Strong<Ctx> as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<'a, 'b, Ctx> PartialOrd<$lhs> for $rhs
        where
            Ctx: Validator + PartialEqTransparent + PartialOrdTransparent
        {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<cmp::Ordering> {
                <Strong<Ctx> as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(StrongBuf<Ctx>, Strong<Ctx>);
impl_cmp!(StrongBuf<Ctx>, &'a Strong<Ctx>);
impl_cmp!(Cow<'a, Strong<Ctx>>, Strong<Ctx>);
impl_cmp!(Cow<'a, Strong<Ctx>>, &'b Strong<Ctx>);
impl_cmp!(Cow<'a, Strong<Ctx>>, StrongBuf<Ctx>);

impl<Ctx> Eq for Strong<Ctx> where Ctx: Validator + PartialEqTransparent + EqTransparent {}

impl<Ctx> Ord for Strong<Ctx>
where
    Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
{
    #[inline]
    fn cmp(&self, other: &Strong<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}

impl<Ctx> Eq for StrongBuf<Ctx> where Ctx: Validator + PartialEqTransparent + EqTransparent {}

impl<Ctx> Ord for StrongBuf<Ctx>
where
    Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
{
    #[inline]
    fn cmp(&self, other: &StrongBuf<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}
