use crate::{StrongBuf, Validator};

pub use imp::Strong;

mod imp {
    #[cfg(feature = "diesel")]
    use crate::impl_diesel::SqlStrong;
    use crate::Validator;
    use std::marker::PhantomData;

    /// Strongly typed [`str`]
    #[allow(non_camel_case_types)]
    #[cfg_attr(
        feature = "diesel",
        derive(AsExpression),
        sql_type = "SqlStrong<Ctx>",
        diesel(not_sized)
    )]
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

        // const?
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

    pub fn count_chars(&self) -> usize { self.as_str().chars().count() }

    /// Map with function: &str -> &str and validation
    pub fn map<F>(&self, f: F) -> Result<&Strong<Ctx>, Ctx::Err>
    where
        F: FnOnce(&str) -> &str
    {
        let s = f(self.as_str());
        Strong::validate(s)
    }

    pub fn trim(&self) -> Result<&Strong<Ctx>, Ctx::Err> { self.map(str::trim) }

    pub fn trim_start(&self) -> Result<&Strong<Ctx>, Ctx::Err> { self.map(str::trim_start) }

    pub fn trim_end(&self) -> Result<&Strong<Ctx>, Ctx::Err> { self.map(str::trim_end) }

    // TODO: Should I implement str methods?
}

impl<Ctx> ToOwned for Strong<Ctx>
where
    Ctx: Validator
{
    type Owned = StrongBuf<Ctx>;
    #[inline]
    fn to_owned(&self) -> StrongBuf<Ctx> {
        unsafe { StrongBuf::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }
    // clone_into
}
