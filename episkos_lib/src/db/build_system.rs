use sqlx::{query, Row};

use crate::metadata::BuildSystem;

use super::{DatabaseObject, Result};

impl<'c> DatabaseObject<'c> for BuildSystem {
    type Id = i32;
    const TABLE_NAME: &'c str = "build_system";

    async fn write_to_db(&self, executor: impl sqlx::AnyExecutor<'_>) -> Result<()> {
        query("INSERT INTO build_system(name, version) VALUES(?, ?)")
            .bind(&self.name)
            .bind(&self.version)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn from_db(id: Self::Id, executor: impl sqlx::AnyExecutor<'_>) -> Result<Self> {
        let row = query("SELECT id, name, version FROM build_system WHERE id = ?")
            .bind(&id)
            .fetch_one(executor)
            .await?;

        let id: i32 = row.try_get("id")?;
        let name: &str = row.try_get("name")?;
        let version: &str = row.try_get("version")?;

        Ok(BuildSystem::new(name, version).with_id(id))
    }

    fn id(&self) -> Result<Self::Id> {
        self.id
    }
}
