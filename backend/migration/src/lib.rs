pub use sea_orm_migration::prelude::*;
use std::env;

mod migrations;
use crate::migrations::users;
use crate::migrations::admin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());

        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![
            Box::new(users::users::Migration),
            Box::new(users::address::Migration),
            Box::new(users::organisation::Migration),
            Box::new(users::roles::Migration),

            Box::new(admin::site::Migration),
            Box::new(admin::admin_users::Migration),
            Box::new(admin::admin_roles::Migration),
            Box::new(admin::users_roles_assignements::Migration),
            Box::new(admin::permissions::Migration),
            Box::new(admin::admin_entities::Migration),
            Box::new(admin::roles_permissions_assignements_entities::Migration),
            Box::new(admin::users_permissions_assignements_entities::Migration),

            Box::new(admin::data_seed::add_roles::Migration),
            Box::new(admin::data_seed::add_permissions::Migration),
            Box::new(admin::data_seed::add_entities::Migration),
        ];

        match environment.as_str() {
            "development" => {
                println!("Development environment, using development migrations");

                migrations.push(Box::new(users::data_seed::development::add_users::Migration));

                migrations.push(Box::new(admin::data_seed::development::add_admin_users::Migration));
                migrations.push(Box::new(admin::data_seed::development::add_users_roles_assignements::Migration));
                migrations.push(Box::new(admin::data_seed::development::add_roles_permissions_assignements::Migration));
                migrations.push(Box::new(admin::data_seed::development::add_users_permissions_assignements::Migration));
                migrations.push(Box::new(admin::data_seed::development::add_sites::Migration));
            },
            "production" => {
                println!("Production environment, using default migrations");
            },
            _ => {
                println!("Unknown environment, using default migrations");
            }
        }

        migrations
    }
}
