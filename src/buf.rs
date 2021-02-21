use crate::{Strong, Validator};
use std::{marker::PhantomData, ops::Deref};

/// Strongly typed [`String`]
pub struct StrongBuf<Ctx: Validator> {
    phantom: PhantomData<Ctx>,
    inner: String
}

impl<Ctx: Validator> StrongBuf<Ctx> {
    /// Constructs from [`String`].
    #[inline]
    pub fn validate(s: String) -> Result<Self, Ctx::Err> {
        Ctx::validate(&s)?;
        Ok(unsafe { Self::without_validate(s) })
    }

    #[inline]
    pub unsafe fn without_validate(s: String) -> Self {
        Self {
            inner: s,
            phantom: PhantomData
        }
    }

    /// Re-validates self
    pub fn valid(self) -> Result<Self, Ctx::Err> {
        Ctx::validate(&self.inner)?;
        Ok(self)
    }

    /// Converts to [`String`].
    pub fn into_string(self) -> String { self.inner }

    #[inline]
    pub(crate) unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        Self::without_validate(String::from_utf8_unchecked(bytes))
    }

    #[inline]
    pub fn as_strong(&self) -> &Strong<Ctx> { self }

    // TODO: Should I implement String methods?
}

impl<Ctx: Validator> Deref for StrongBuf<Ctx> {
    type Target = Strong<Ctx>;
    #[inline]
    fn deref(&self) -> &Strong<Ctx> { unsafe { Strong::without_validate(&self.inner) } }
}

impl<Ctx: Validator> std::borrow::Borrow<Strong<Ctx>> for StrongBuf<Ctx> {
    #[inline]
    fn borrow(&self) -> &Strong<Ctx> { self.deref() }
}

mod impl_clone;
mod impl_default;
mod impl_fmt;
mod impl_hash;
mod impl_ord;
