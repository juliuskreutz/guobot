use anyhow::Result;
use sqlx::SqlitePool;

pub struct DbEntry {
    pub id: String,
    pub name: String,
    pub url: String,
    pub message: Option<String>,
}

pub async fn get_entry_by_id(id: &str, pool: &SqlitePool) -> Result<DbEntry> {
    Ok(
        sqlx::query_as!(DbEntry, "SELECT * FROM entries WHERE id == ?1", id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_entries(pool: &SqlitePool) -> Result<Vec<DbEntry>> {
    Ok(sqlx::query_as!(DbEntry, "SELECT * FROM entries")
        .fetch_all(pool)
        .await?)
}

pub async fn set_entry(entry: &DbEntry, pool: &SqlitePool) -> Result<()> {
    sqlx::query!(
        "INSERT OR REPLACE INTO entries(id, name, url, message) VALUES(?, ?, ?, ?)",
        entry.id,
        entry.name,
        entry.url,
        entry.message
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_entry_by_id(id: &str, pool: &SqlitePool) -> Result<()> {
    sqlx::query!("DELETE FROM entries WHERE id = ?1", id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn set_entry_message_by_id(id: &str, message: &str, pool: &SqlitePool) -> Result<()> {
    sqlx::query!("UPDATE entries SET message = ?1 WHERE id = ?2", message, id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_entry_message_by_id(id: &str, pool: &SqlitePool) -> Result<()> {
    sqlx::query!("UPDATE entries SET message = NULL WHERE id = ?1", id)
        .execute(pool)
        .await?;

    Ok(())
}
