use async_graphql::{MergedObject, Object};

use crate::internal::api::{admin, users};

#[derive(MergedObject, Default)]
pub struct AdminMutationRoot(
    pub admin::users::controllers::auth::AuthAdminMutation
);

#[derive(MergedObject, Default)]
pub struct UserMutationRoot(
    pub users::controllers::UserMutation
);

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn admin(&self) -> AdminMutationRoot {
        AdminMutationRoot::default()
    }

    async fn users(&self) -> UserMutationRoot {
        UserMutationRoot::default()
    }
}
