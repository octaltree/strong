use crate::{StrongBuf, Validator};

pub use imp::Strong;

mod imp {
    use crate::Validator;
    use std::marker::PhantomData;

    /// Strongly typed [`str`]
    #[allow(non_camel_case_types)]
    #[repr(transparent)]
    pub struct Strong<Ctx: Validator> {
        phantom: PhantomData<Ctx>,
        inner: str
    }

    impl<Ctx: Validator> Strong<Ctx> {
        /// Constructs from [`str`].
        #[inline]
        pub fn validate<S: AsRef<str> + ?Sized>(s: &S) -> Result<&Self, Ctx::Err> {
            Ctx::validate(s.as_ref())?;
            Ok(unsafe { Self::no_validate(s) })
        }

        /// Constructs from [`str`] without validation.
        /// ## Safety
        /// This function allows us to create invalid [`Strong`].
        #[inline]
        pub unsafe fn no_validate<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
            &*(s.as_ref() as *const str as *const Self)
        }

        /// Converts to [`str`].
        #[inline]
        pub fn as_str(&self) -> &str { &self.inner }

        // TODO: const?
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] { self.inner.as_bytes() }
    }
}

impl<Ctx: Validator> Strong<Ctx> {
    /// Re-validates self
    pub fn valid(&self) -> Result<&Self, Ctx::Err> {
        Ctx::validate(self.as_str())?;
        Ok(&self)
    }

    /// Converts to [`StrongBuf`].
    pub fn to_strong_buf(&self) -> StrongBuf<Ctx> {
        unsafe { StrongBuf::no_validate(self.as_str().to_string()) }
    }
}

impl<Ctx: Validator> ToOwned for Strong<Ctx> {
    type Owned = StrongBuf<Ctx>;
    #[inline]
    fn to_owned(&self) -> StrongBuf<Ctx> {
        unsafe { StrongBuf::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }
    // TODO: clone_into
}

mod impl_fmt;
mod impl_hash;
mod impl_ord;
