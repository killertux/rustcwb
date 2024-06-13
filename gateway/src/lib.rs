use anyhow::{anyhow, Result};
use domain::{
    GetPastMeetUpError, ListPastMeetUpsError, PastMeetUp, PastMeetUpGateway, PastMeetUpMetadata,
};
use sqlx::{sqlite::SqliteRow, Error, Row, SqlitePool};
use ulid::Ulid;
use url::Url;

pub struct SqliteDatabaseGateway {
    sqlite_pool: SqlitePool,
}

impl SqliteDatabaseGateway {
    pub async fn new(database_url: String) -> Result<Self> {
        let sqlite_pool = SqlitePool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&sqlite_pool).await?;
        Ok(Self { sqlite_pool })
    }
}

impl PastMeetUpGateway for SqliteDatabaseGateway {
    async fn list_past_meet_ups(&self) -> Result<Vec<PastMeetUpMetadata>, ListPastMeetUpsError> {
        Ok(
            sqlx::query("SELECT id, title, date FROM past_meet_ups ORDER BY date desc;")
                .try_map(|row: SqliteRow| {
                    Ok(PastMeetUpMetadata::new(
                        Ulid::from_bytes(
                            row.try_get::<&[u8], _>("id")?
                                .try_into()
                                .map_err(|err| sqlx::Error::Decode(Box::new(err)))?,
                        ),
                        row.get("title"),
                        row.get("date"),
                    ))
                })
                .fetch_all(&self.sqlite_pool)
                .await
                .map_err(|err| anyhow!("SQLX Error: {err}"))?,
        )
    }

    async fn get_past_meet_up(&self, id: Ulid) -> Result<PastMeetUp, GetPastMeetUpError> {
        sqlx::query("SELECT * FROM past_meet_ups WHERE id = ?")
            .bind(id.to_bytes().as_slice())
            .try_map(|row: SqliteRow| {
                Ok(PastMeetUp::new(
                    Ulid::from_bytes(
                        row.try_get::<&[u8], _>("id")?
                            .try_into()
                            .map_err(|err| Error::Decode(Box::new(err)))?,
                    ),
                    row.get("title"),
                    row.get("description"),
                    row.get("speaker"),
                    row.get("date"),
                    Url::parse(row.get("link")).map_err(|err| Error::Decode(Box::new(err)))?,
                ))
            })
            .fetch_one(&self.sqlite_pool)
            .await
            .map_err(|err| match err {
                Error::RowNotFound => GetPastMeetUpError::NotFound(id),
                _ => GetPastMeetUpError::Unknown(anyhow!("SQLX Error: {err}")),
            })
    }
}