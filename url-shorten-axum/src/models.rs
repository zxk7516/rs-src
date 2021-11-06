use crate::{
    DBPool,
    QueryResult,
};
use serde::Serialize;
use sqlx::Error;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct ShortLinks {
    pub id: i32,
    pub url: String,
}

#[derive(Serialize, FromRow, Clone, Copy)]
pub struct ReturningId<T>
where
    T: Copy,
{
    pub id: T,
}

impl<T> ReturningId<T>
where
    T: Copy,
{
    #[inline]
    pub fn last_insert_id(&self) -> T {
        self.id
    }
}

// INSERT INTO `short_links`.`short_links` (`url`) VALUES ('ABC')
#[cfg(feature = "mysql")]
pub async fn create_short_link(
    pool: &DBPool,
    url: &str,
) -> Result<i32, Error> {
    let exists: Result<ReturningId<i32>, Error> =
        sqlx::query_as(r#"select id from short_links where url = ?"#)
            .bind(url)
            .fetch_one(pool)
            .await;
    if exists.is_ok() {
        Ok(exists.unwrap().id)
    } else {
        sqlx::query(
            r#"
INSERT INTO short_links (`url`) VALUES (?)
"#,
        )
        .bind(url)
        .execute(pool)
        .await
        .map(|r| r.last_insert_id() as i32)
    }
}

#[cfg(feature = "postgres")]
pub async fn create_short_link(
    pool: &DBPool,
    url: &str,
) -> Result<i32, Error> {
    use sqlx::Executor;

    let exists: Result<ReturningId<i32>, Error> =
        sqlx::query_as(r#"select id from short_links where url = $1"#)
            .bind(url)
            .fetch_one(pool)
            .await;
    if exists.is_ok() {
        Ok(exists.unwrap().id)
    } else {
        let insert_res: Result<ReturningId<i32>, Error> = sqlx::query_as(
            //         r#"
            // INSERT INTO short_links ("url") VALUES ($1) ON CONFLICT ON CONSTRAINT short_links_url_key DO UPDATE set url = excluded.url returning id
            // "#,
            r#"
with e as (
INSERT INTO short_links ("url") VALUES ($1) ON CONFLICT DO NOTHING returning id
) select COALESCE(
(select id from e),
(SELECT id FROM short_links where "url" = $1)
) as id
"#,
        )
        .bind(url)
        .fetch_one(pool)
        .await;
        insert_res.map(|r| r.id)
    }
}

// DELETE FROM `short_links`.`short_links` WHERE `id` = 1
pub async fn delete_short_link(
    pool: &DBPool,
    id: i32,
) -> Result<QueryResult, Error> {
    sqlx::query(
        #[cfg(feature = "mysql")]
        r#" DELETE FROM short_links where id = ?"#,
        #[cfg(feature = "postgres")]
        r#" DELETE FROM short_links where id = $1"#,
    )
    .bind(id)
    .execute(pool)
    .await
}

pub async fn get_short_link(
    pool: &DBPool,
    id: i32,
) -> Result<ShortLinks, Error> {
    sqlx::query_as(
        #[cfg(feature = "mysql")]
        r#" select id, url FROM short_links where id = ?"#,
        #[cfg(feature = "postgres")]
        r#" select id, url FROM short_links where id = $1"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
