use crate::{marker, StrongBuf, Validator};
use diesel::pg::Pg;
use std::marker::PhantomData;

#[derive(SqlType, QueryId)]
#[postgres(oid = "25", array_oid = "1009")]
pub struct SqlStrong<Ctx>(PhantomData<Ctx>);

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

// TODO: AsExpression
