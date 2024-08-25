use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Chat {
    Table,
    Id,
}
#[derive(DeriveIden)]
pub enum GlobalRegex {
    Table,
    Id,
    Regexp,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GlobalRegex::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(GlobalRegex::Regexp).text().not_null())
                    .col(
                        ColumnDef::new(GlobalRegex::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .text()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}
