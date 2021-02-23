use crate::{Strong, Validator};

pub use imp::StrongBuf;

mod imp {
    #[cfg(feature = "diesel")]
    use crate::impl_diesel::SqlStrong;
    use crate::{Strong, Validator};
    use std::{marker::PhantomData, ops::Deref};

    /// Strongly typed [`String`]
    #[cfg_attr(
        feature = "diesel",
        derive(FromSqlRow, AsExpression),
        sql_type = "SqlStrong<Ctx>"
    )]
    pub struct StrongBuf<Ctx: Validator> {
        phantom: PhantomData<Ctx>,
        inner: String
    }

    impl<Ctx: Validator> StrongBuf<Ctx> {
        /// Constructs from [`String`].
        #[inline]
        pub fn validate(s: String) -> Result<Self, Ctx::Err> {
            Ctx::validate(&s)?;
            Ok(unsafe { Self::no_validate(s) })
        }

        /// Constructs from [`String`] without validation.
        /// ## Safety
        /// This function allows us to create invalid [`StrongBuf`].
        #[inline]
        pub unsafe fn no_validate(s: String) -> Self {
            Self {
                inner: s,
                phantom: PhantomData
            }
        }

        /// Converts to [`String`].
        #[inline]
        pub fn into_string(self) -> String { self.inner }

        #[inline]
        pub fn as_strong(&self) -> &Strong<Ctx> { self }
    }

    impl<Ctx> Deref for StrongBuf<Ctx>
    where
        Ctx: Validator
    {
        type Target = Strong<Ctx>;
        #[inline]
        fn deref(&self) -> &Strong<Ctx> { unsafe { Strong::no_validate(&self.inner) } }
    }
}

impl<Ctx: Validator> StrongBuf<Ctx> {
    /// Constructs from [`Vec<u8>`] without validation.
    /// ## Safety
    /// This function allows us to create invalid [`StrongBuf`].
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        Self::no_validate(String::from_utf8_unchecked(bytes))
    }

    // TODO: Should I implement String methods?
}

impl<Ctx> std::borrow::Borrow<Strong<Ctx>> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    #[inline]
    fn borrow(&self) -> &Strong<Ctx> { self }
}

impl<Ctx> Clone for StrongBuf<Ctx>
where
    Ctx: Validator
{
    #[inline]
    fn clone(&self) -> Self { unsafe { Self::from_utf8_unchecked(self.as_bytes().to_owned()) } }
}
