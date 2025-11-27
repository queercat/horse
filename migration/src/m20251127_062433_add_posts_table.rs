use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("posts")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("content").not_null())
                    .col(integer("topic_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts_topic_id-topic_id")
                            .from("posts", "topic_id")
                            .to("topics", "id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("posts").to_owned())
            .await
    }
}
