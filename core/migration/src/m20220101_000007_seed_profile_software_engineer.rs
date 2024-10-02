use sea_orm_migration::prelude::*;

use crate::m20220101_000006_create_profile_table::Profile;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub static SOFTWARE_ENGINEER_PROFILE_NAME: &str = "Senior Software Engineer";
static SOFTWARE_ENGINEER_PROFILE_PROMPT: &str = r#"
# audience
A senior software developer.

# style
Be straight forward and concise. Only give explanation if asked.

## References
When the answer contains an external project, dependency, command line tools, application or executable, a library or any external references: ALWAYS provide sources and give an URL to the reference. Prefer sources of how to use and install.

## Code Format
When asked about code questions, give code example.
Provide library answer only if the question is explicitly about code and a language is specified.
If an existing library (or many libraries) already exist for the question, provide it. 
"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert: InsertStatement = Query::insert()
            .into_table(Profile::Table)
            .columns([Profile::Name, Profile::Prompt])
            .values_panic([
                SOFTWARE_ENGINEER_PROFILE_NAME.into(),
                SOFTWARE_ENGINEER_PROFILE_PROMPT.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Profile::Table)
            .and_where(Expr::col(Profile::Name).eq(SOFTWARE_ENGINEER_PROFILE_NAME))
            .to_owned();

        manager.exec_stmt(delete).await
    }
}
