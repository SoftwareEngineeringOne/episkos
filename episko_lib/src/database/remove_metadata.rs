use crate::metadata::Metadata;

use super::{DatabaseHandler, Result};

impl Metadata {
    /// Remove a given Metadata object from the cache. This does not
    /// delete it's manifest file.
    pub async fn remove_from_db(&self, db: &DatabaseHandler) -> Result<()> {
        sqlx::query("DELETE FROM metadata WHERE id = ?")
            .bind(self.id)
            .execute(db.conn())
            .await?;
        Ok(())
    }
}
