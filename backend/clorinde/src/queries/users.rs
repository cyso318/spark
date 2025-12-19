// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct InsertUser {
    pub id: i32,
    pub username: String,
}
pub struct InsertUserBorrowed<'a> {
    pub id: i32,
    pub username: &'a str,
}
impl<'a> From<InsertUserBorrowed<'a>> for InsertUser {
    fn from(InsertUserBorrowed { id, username }: InsertUserBorrowed<'a>) -> Self {
        Self {
            id,
            username: username.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetAllUsers {
    pub id: i32,
    pub username: String,
}
pub struct GetAllUsersBorrowed<'a> {
    pub id: i32,
    pub username: &'a str,
}
impl<'a> From<GetAllUsersBorrowed<'a>> for GetAllUsers {
    fn from(GetAllUsersBorrowed { id, username }: GetAllUsersBorrowed<'a>) -> Self {
        Self {
            id,
            username: username.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct InsertUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<InsertUserBorrowed, tokio_postgres::Error>,
    mapper: fn(InsertUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> InsertUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(InsertUserBorrowed) -> R,
    ) -> InsertUserQuery<'c, 'a, 's, C, R, N> {
        InsertUserQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetAllUsersQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetAllUsersBorrowed, tokio_postgres::Error>,
    mapper: fn(GetAllUsersBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetAllUsersQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetAllUsersBorrowed) -> R,
    ) -> GetAllUsersQuery<'c, 'a, 's, C, R, N> {
        GetAllUsersQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct InsertUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_user() -> InsertUserStmt {
    InsertUserStmt(
        "INSERT INTO users (username) VALUES ($1) RETURNING id, username",
        None,
    )
}
impl InsertUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        username: &'a T1,
    ) -> InsertUserQuery<'c, 'a, 's, C, InsertUser, 1> {
        InsertUserQuery {
            client,
            params: [username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<InsertUserBorrowed, tokio_postgres::Error> {
                    Ok(InsertUserBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                    })
                },
            mapper: |it| InsertUser::from(it),
        }
    }
}
pub struct GetAllUsersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_all_users() -> GetAllUsersStmt {
    GetAllUsersStmt("SELECT id, username FROM users", None)
}
impl GetAllUsersStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> GetAllUsersQuery<'c, 'a, 's, C, GetAllUsers, 0> {
        GetAllUsersQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetAllUsersBorrowed, tokio_postgres::Error> {
                    Ok(GetAllUsersBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                    })
                },
            mapper: |it| GetAllUsers::from(it),
        }
    }
}
