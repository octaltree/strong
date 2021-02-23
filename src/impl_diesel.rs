use crate::{marker, Strong, StrongBuf, Validator};
use diesel::pg::Pg;
use std::marker::PhantomData;

#[derive(SqlType, QueryId)]
#[postgres(oid = "25", array_oid = "1009")]
pub struct SqlStrong<Ctx>(PhantomData<Ctx>);

impl<Ctx> Default for SqlStrong<Ctx> {
    fn default() -> Self { SqlStrong(PhantomData) }
}

impl<Ctx> diesel::serialize::ToSql<SqlStrong<Ctx>, Pg> for StrongBuf<Ctx>
where
    Ctx: Validator + marker::DebugTransparent
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::<diesel::sql_types::Text, Pg>::to_sql::<W>(self.as_str(), out)
    }
}

impl<'a, Ctx> diesel::deserialize::FromSql<SqlStrong<Ctx>, Pg> for StrongBuf<Ctx>
where
    Ctx: Validator,
    Ctx::Err: std::error::Error + Send + Sync + 'static
{
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let s: String =
            diesel::deserialize::FromSql::<diesel::sql_types::Text, Pg>::from_sql(bytes)?;
        Ok(Self::validate(s)?)
    }
}

fn _impl_as_expression() {
    use diesel::{
        expression::{bound::Bound, AsExpression},
        serialize::{self, Output, ToSql},
        sql_types::Nullable
    };
    impl<'expr, Ctx> AsExpression<SqlStrong<Ctx>> for &'expr Strong<Ctx>
    where
        Ctx: Validator
    {
        type Expression = Bound<SqlStrong<Ctx>, Self>;
        fn as_expression(self) -> Self::Expression { Bound::new(self) }
    }
    impl<'expr, Ctx> AsExpression<Nullable<SqlStrong<Ctx>>> for &'expr Strong<Ctx>
    where
        Ctx: Validator
    {
        type Expression = Bound<Nullable<SqlStrong<Ctx>>, Self>;
        fn as_expression(self) -> Self::Expression { Bound::new(self) }
    }
    impl<__DB, Ctx> diesel::serialize::ToSql<Nullable<SqlStrong<Ctx>>, __DB> for Strong<Ctx>
    where
        Ctx: Validator,
        __DB: diesel::backend::Backend,
        Self: ToSql<SqlStrong<Ctx>, __DB>
    {
        fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, __DB>) -> serialize::Result {
            ToSql::<SqlStrong<Ctx>, __DB>::to_sql(self, out)
        }
    }
    impl<Ctx> AsExpression<SqlStrong<Ctx>> for StrongBuf<Ctx>
    where
        Ctx: Validator
    {
        type Expression = Bound<SqlStrong<Ctx>, Self>;
        fn as_expression(self) -> Self::Expression { Bound::new(self) }
    }
    impl<Ctx> AsExpression<Nullable<SqlStrong<Ctx>>> for StrongBuf<Ctx>
    where
        Ctx: Validator
    {
        type Expression = Bound<Nullable<SqlStrong<Ctx>>, Self>;
        fn as_expression(self) -> Self::Expression { Bound::new(self) }
    }
}
