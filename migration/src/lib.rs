pub use sea_orm_migration::prelude::*;
mod m20251122_212357_create_users_table;
mod m20251122_213125_seed_users_table;
mod m20251124_040703_create_topics_table;
mod m20251124_054355_seed_topics_table;
mod m20251127_062433_add_posts_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251122_212357_create_users_table::Migration),
            Box::new(m20251122_213125_seed_users_table::Migration),
            Box::new(m20251124_040703_create_topics_table::Migration),
            Box::new(m20251124_054355_seed_topics_table::Migration),
            Box::new(m20251127_062433_add_posts_table::Migration),
        ]
    }
}
