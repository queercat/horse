use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::insert()
            .into_table("topics")
            .columns(["id", "title", "description"])
            .values_panic([0.into(), "memes".into(), "this is where the memes are".into()])
            .to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::delete().from_table("topics").cond_where(Cond::any().add(Expr::col("id").eq(0))).to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }
}
