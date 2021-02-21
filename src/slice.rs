use crate::{StrongBuf, Validator};
use std::marker::PhantomData;

/// Strongly typed [`str`]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct Strong<Ctx: Validator> {
    phantom: PhantomData<Ctx>,
    inner: str
}

impl<Ctx: Validator> Strong<Ctx> {
    /// construct from [`str`].
    pub fn validate<S: AsRef<str> + ?Sized>(s: &S) -> Result<&Self, Ctx::Err> {
        Ctx::validate(s.as_ref())?;
        Ok(unsafe { Self::no_validate(s) })
    }

    /// Construct from [`str`] without validation.
    /// ## Safety
    /// This function allows us to create invalid [`Strong`].
    pub unsafe fn no_validate<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        &*(s.as_ref() as *const str as *const Self)
    }

    /// Re-validates self
    pub fn valid(&self) -> Result<&Self, Ctx::Err> {
        Ctx::validate(&self.inner)?;
        Ok(&self)
    }

    /// convert to [`str`].
    pub fn as_str(&self) -> &str { &self.inner }

    pub fn to_strong_buf(&self) -> StrongBuf<Ctx> {
        unsafe { StrongBuf::no_validate(self.inner.to_string()) }
    }

    // FIXME: const?
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] { self.inner.as_bytes() }
}

impl<Ctx: Validator> ToOwned for Strong<Ctx> {
    type Owned = StrongBuf<Ctx>;
    #[inline]
    fn to_owned(&self) -> StrongBuf<Ctx> {
        unsafe { StrongBuf::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }
    // TODO: clone_into
}
