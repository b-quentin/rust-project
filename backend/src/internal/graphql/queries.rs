use async_graphql::{MergedObject, Object};

use crate::internal::api::{admin, users};

#[derive(MergedObject, Default)]
pub struct AdminQueryRoot(
    pub admin::users::controllers::auth::AuthAdminQuery,
    pub admin::users::controllers::users::AdminUserQuery
);

#[derive(MergedObject, Default)]
pub struct UserQueryRoot(
    pub users::controllers::UserQuery
);

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn admin(&self) -> AdminQueryRoot {
        AdminQueryRoot::default()
    }

    async fn user(&self) -> UserQueryRoot {
        UserQueryRoot::default()
    }
}
