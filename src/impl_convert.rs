//! impl From, TryFrom
use crate::{Strong, StrongBuf, Validator};
use std::{convert::TryFrom, str::FromStr};

impl<Ctx> TryFrom<String> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    type Error = Ctx::Err;
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::validate(s) }
}

impl<'a, Ctx> TryFrom<&'a str> for &'a Strong<Ctx>
where
    Ctx: Validator
{
    type Error = Ctx::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> { Strong::validate(s) }
}

impl<Ctx> FromStr for StrongBuf<Ctx>
where
    Ctx: Validator
{
    type Err = Ctx::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::validate(s.to_string()) }
}

impl<Ctx> AsRef<Strong<Ctx>> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    fn as_ref(&self) -> &Strong<Ctx> { self }
}

fn _impl_from() {
    use std::{borrow::Cow, rc::Rc, sync::Arc};

    /// Needs allocation
    impl<Ctx> From<&Strong<Ctx>> for Box<Strong<Ctx>>
    where
        Ctx: Validator
    {
        fn from(s: &Strong<Ctx>) -> Self {
            let boxed: Box<str> = s.as_str().into();
            let rw = Box::into_raw(boxed) as *mut Strong<Ctx>;
            unsafe { Box::from_raw(rw) }
        }
    }

    impl<Ctx> From<Box<Strong<Ctx>>> for StrongBuf<Ctx>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(boxed: Box<Strong<Ctx>>) -> StrongBuf<Ctx> {
            let rw = Box::into_raw(boxed) as *mut str;
            let inner: Box<str> = unsafe { Box::from_raw(rw) };
            unsafe { StrongBuf::no_validate(String::from(inner)) }
        }
    }

    impl<Ctx> From<StrongBuf<Ctx>> for Box<Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(s: StrongBuf<Ctx>) -> Self { s.into_boxed_strong() }
    }

    impl<Ctx> From<Cow<'_, Strong<Ctx>>> for Box<Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(cow: Cow<'_, Strong<Ctx>>) -> Box<Strong<Ctx>> {
            match cow {
                Cow::Borrowed(s) => Box::from(s),
                Cow::Owned(s) => Box::from(s)
            }
        }
    }

    impl<'a, Ctx> From<&'a Strong<Ctx>> for Cow<'a, Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(s: &'a Strong<Ctx>) -> Cow<'a, Strong<Ctx>> { Cow::Borrowed(s) }
    }

    impl<'a, Ctx> From<StrongBuf<Ctx>> for Cow<'a, Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(s: StrongBuf<Ctx>) -> Cow<'a, Strong<Ctx>> { Cow::Owned(s) }
    }

    impl<Ctx> From<StrongBuf<Ctx>> for Arc<Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(s: StrongBuf<Ctx>) -> Self {
            let a: Arc<str> = Arc::from(s.into_string());
            let rw = Arc::into_raw(a) as *const Strong<Ctx>;
            unsafe { Arc::from_raw(rw) }
        }
    }

    impl<Ctx> From<StrongBuf<Ctx>> for Rc<Strong<Ctx>>
    where
        Ctx: Validator
    {
        #[inline]
        fn from(s: StrongBuf<Ctx>) -> Self {
            let a: Rc<str> = Rc::from(s.into_string());
            let rw = Rc::into_raw(a) as *const Strong<Ctx>;
            unsafe { Rc::from_raw(rw) }
        }
    }

    /// Needs allocation
    impl<Ctx> From<&Strong<Ctx>> for Arc<Strong<Ctx>>
    where
        Ctx: Validator
    {
        fn from(s: &Strong<Ctx>) -> Self {
            let boxed: Arc<str> = s.as_str().into();
            let rw = Arc::into_raw(boxed) as *const Strong<Ctx>;
            unsafe { Arc::from_raw(rw) }
        }
    }

    /// Needs allocation
    impl<Ctx> From<&Strong<Ctx>> for Rc<Strong<Ctx>>
    where
        Ctx: Validator
    {
        fn from(s: &Strong<Ctx>) -> Self {
            let boxed: Rc<str> = s.as_str().into();
            let rw = Rc::into_raw(boxed) as *const Strong<Ctx>;
            unsafe { Rc::from_raw(rw) }
        }
    }

    // Needs allocation
    // 1460: impl<T: ?Sized + AsRef<OsStr>> From<&T> for PathBuf {
    // 1588: impl<'a> From<Cow<'a, Path>> for PathBuf {
}
