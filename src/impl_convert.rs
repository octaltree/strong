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
    use std::borrow::Cow;

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
}

// TODO: Box, Rc, Arc, Cow
// 1460: impl<T: ?Sized + AsRef<OsStr>> From<&T> for PathBuf {
// 1468: impl From<OsString> for PathBuf {
// 1490: impl From<String> for PathBuf {
// 1501: impl FromStr for PathBuf {
// 1511: impl<P: AsRef<Path>> iter::FromIterator<P> for PathBuf {
// 1564: impl<'a> From<&'a Path> for Cow<'a, Path> {
// 1572: impl<'a> From<PathBuf> for Cow<'a, Path> {
// 1580: impl<'a> From<&'a PathBuf> for Cow<'a, Path> {
// 1588: impl<'a> From<Cow<'a, Path>> for PathBuf {
// 1596: impl From<PathBuf> for Arc<Path> {
// 1606: impl From<&Path> for Arc<Path> {
// 1616: impl From<PathBuf> for Rc<Path> {
// 1626: impl From<&Path> for Rc<Path> {

// impl<'_> From<&'_ String> for String
// impl<'_> From<&'_ mut str> for String
// impl<'_> From<&'_ str> for String
// impl<'a> From<&'a String> for Cow<'a, str>
// impl<'a> From<Cow<'a, str>> for String
// impl From<String> for Rc<str>
// impl From<String> for Vec<u8, Global>
// impl<'a> From<String> for Cow<'a, str>
// impl From<String> for Arc<str>
// impl From<String> for Box<dyn Error + Send + Sync>
// impl From<String> for Box<dyn Error>
// impl From<String> for OsString
// impl From<String> for PathBuf
// impl From<char> for String
