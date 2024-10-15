use std::sync::Arc;

use async_graphql::{
    EmptyMutation,
    EmptySubscription,
    Schema
};
use sea_orm::{sqlx::types::chrono::Utc, DatabaseBackend, DbErr, MockDatabase, MockExecResult};
use uuid::Uuid;
use crate::internal::api::users::{
    controllers::graphql::{users::{CreateUserInput, UpdateUserInput}, MutationRoot, QueryRoot},
    models::users
};

#[tokio::test]
async fn test_user_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database with a matching user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![users::Model {
                id: fixed_uuid,
                username: "test_user".to_owned(),
                first_name: "test".to_owned(),
                last_name: "user".to_owned(),
                email: "test@example.com".to_owned(),
                password: "hashed_password".to_owned(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
            }],
        ])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Execute the query
    let query = format!(r#"{{
        user(id: "{}") {{
            id
            username
            email
        }}
    }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.is_ok());
    let data = response.data.into_json().unwrap();
    assert_eq!(data["user"]["id"], fixed_uuid.to_string());
    assert_eq!(data["user"]["username"], "test_user");
    assert_eq!(data["user"]["email"], "test@example.com");
}

#[tokio::test]
async fn test_user_not_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate no user found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<users::Model, Vec<users::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Execute the query
    let query = format!(r#"{{
        user(id: "{}") {{
            id
            username
            email
        }}
    }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.is_err());
    let errors = response.errors;
    assert!(errors.iter().any(|e| e.message == format!("User with id {} not found", fixed_uuid)));
}

#[tokio::test]
async fn test_user_db_error() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to return an error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Database error".into())])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Execute the query
    let query = format!(r#"{{
        user(id: "{}") {{
            id
            username
            email
        }}
    }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.is_err());
    let errors = response.errors;
    assert!(errors.iter().any(|e| e.message.contains("Failed to fetch user with error")));
}

#[tokio::test]
async fn test_users_found() {
    // Mock UUIDs for testing
    let uuid1 = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();
    let uuid2 = Uuid::parse_str("f30c1f8f-55c8-4ad5-b3e8-4f4530a73a58").unwrap();

    // Mock the database with multiple users
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![
            users::Model {
                id: uuid1,
                username: "test_user1".to_owned(),
                email: "test1@example.com".to_owned(),
                password: "hashed_password1".to_owned(),
                first_name: "user".to_owned(),
                last_name: "test".to_owned(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
            },
            users::Model {
                id: uuid2,
                username: "test_user2".to_owned(),
                email: "test2@example.com".to_owned(),
                password: "hashed_password2".to_owned(),
                first_name: "user".to_owned(),
                last_name: "test".to_owned(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
            },
        ]])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Execute the query to fetch all users
    let query = format!(r#"
        {{
            users {{
                id
                username
                email
            }}
        }}
    "#);

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.errors.is_empty(), "Unexpected errors: {:?}", response.errors);
    let data = response.data.into_json().unwrap();

    // Assertions for multiple users
    assert_eq!(data["users"][0]["id"], uuid1.to_string());
    assert_eq!(data["users"][0]["username"], "test_user1");
    assert_eq!(data["users"][0]["email"], "test1@example.com");

    assert_eq!(data["users"][1]["id"], uuid2.to_string());
    assert_eq!(data["users"][1]["username"], "test_user2");
    assert_eq!(data["users"][1]["email"], "test2@example.com");
}

#[tokio::test]
async fn test_users_not_found() {
    // Mock the database to simulate no users found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<users::Model, Vec<users::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Use format! to dynamically create the query
    let query = format!(r#"
        {{
            users {{
                id
                username
                email
            }}
        }}
    "#);

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.errors.is_empty(), "Unexpected errors: {:?}", response.errors);
    let data = response.data.into_json().unwrap();

    // Assertions to confirm no users are returned
    assert!(data["users"].is_array());
    assert!(data["users"].as_array().unwrap().is_empty(), "Expected no users but found some.");
}

#[tokio::test]
async fn test_users_db_error() {
    // Mock the database to simulate a database error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Simulated database connection error".into())]) // Simulate a DB error
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    // Use format! to dynamically create the query
    let query = format!(r#"
        {{
            users {{
                id
                username
                email
            }}
        }}
    "#);

    let response = schema.execute(&query).await;

    // Assert the response for expected errors
    assert!(!response.errors.is_empty(), "Expected errors but found none.");
    let errors = response.errors;

    // Assertions to confirm the error message
    assert!(
        errors.iter().any(|e| e.message.contains("Failed to fetch users with error")),
        "Expected database connection error but found: {:?}",
        errors
    );
}

#[tokio::test]
async fn test_create_user_success() {
    // Mock input for creating a user
    let input = CreateUserInput {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        first_name: "test".to_string(),
        last_name: "user".to_string(),
    };

    // Fixed UUID for the new user
    let generated_uuid = Uuid::new_v4();

    // Mock the database to simulate successful user creation
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful insertion
            last_insert_id: 0, // Make sure this is set correctly if used
        }])
        .append_query_results([vec![users::Model {
            id: generated_uuid,
            username: input.username.clone(),
            email: input.email.clone(),
            password: "hashed_password".to_owned(),
            first_name: input.first_name.clone(),
            last_name: input.last_name.clone(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }]])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            createUser(input: {{
                username: "{}",
                email: "{}",
                password: "{}"
                firstName: "{}",
                lastName: "{}"
            }}) {{
                id
                username
                email
                firstName
                lastName
            }}
        }}"#, input.username, input.email, input.password, input.first_name, input.last_name);

    println!("Query: {}", query);

    let response = schema.execute(&query).await;

    // Debug print response for troubleshooting
    println!("Response: {:?}", response);

    // Assert the response
    assert!(response.errors.is_empty(), "Unexpected errors: {:?}", response.errors);
    let data = response.data.into_json().unwrap();

    // Assertions
    assert_eq!(data["createUser"]["id"], generated_uuid.to_string(), "User ID does not match");
    assert_eq!(data["createUser"]["username"], input.username, "Username does not match");
    assert_eq!(data["createUser"]["email"], input.email, "Email does not match");
}

#[tokio::test]
async fn test_create_user_db_error() {
    // Mock input for creating a user
    let input = CreateUserInput {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        first_name: "test".to_string(),
        last_name: "user".to_string(),
    };

    // Mock the database to return an error during user creation
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<users::Model, Vec<users::Model>, _>([vec![]]) // No prior users in the system
        .append_exec_errors([DbErr::Custom("Insertion error".into())]) // Simulate a database error
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            createUser(input: {{
                username: "{}",
                email: "{}",
                password: "{}"
                firstName: "{}",
                lastName: "{}"
            }}) {{
                id
                username
                email
                firstName
                lastName
            }}
        }}"#, input.username, input.email, input.password, input.first_name, input.last_name);

    let response = schema.execute(&query).await;

    // Debug print response for troubleshooting
    println!("Response: {:?}", response);

    // Assert the response
    assert!(!response.errors.is_empty(), "Expected errors but got none");
    let errors = response.errors;
    assert!(errors.iter().any(|e| e.message.contains("Failed to create user with username")), "Expected error message not found");
}

#[tokio::test]
async fn test_update_user_success() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock input for updating a user
    let input = UpdateUserInput {
        id: fixed_uuid,
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: None,
    };

    // Mock the database to simulate successful user update
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: fixed_uuid,
            username: "original_user".to_owned(),
            email: "original@example.com".to_owned(),
            password: "hashed_password".to_owned(),
            first_name: "original".to_owned(),
            last_name: "user".to_owned(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }]]) // Simulate finding the user
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful update
            last_insert_id: 0,
        }])
        .append_query_results([vec![users::Model {
            id: fixed_uuid,
            username: input.username.clone().unwrap(),
            email: input.email.clone().unwrap(),
            password: "hashed_password".to_owned(),
            first_name: "original".to_owned(),
            last_name: "user".to_owned(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }]]) // Simulate returning the updated user
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            updateUser(input: {{
                id: "{}",
                username: "{}",
                email: "{}"
            }}) {{
                id
                username
                email
            }}
        }}"#, input.id, input.username.clone().unwrap(), input.email.clone().unwrap());

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(response.errors.is_empty());
    let data = response.data.into_json().unwrap();
    assert_eq!(data["updateUser"]["id"], fixed_uuid.to_string());
    assert_eq!(data["updateUser"]["username"], input.username.unwrap());
    assert_eq!(data["updateUser"]["email"], input.email.unwrap());
}

#[tokio::test]
async fn test_update_user_not_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock input for updating a user
    let input = UpdateUserInput {
        id: fixed_uuid,
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: None,
    };

    // Mock the database to simulate user not found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<users::Model, Vec<users::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            updateUser(input: {{
                id: "{}",
                username: "{}",
                email: "{}"
            }}) {{
                id
                username
                email
            }}
        }}"#, input.id, input.username.clone().unwrap(), input.email.clone().unwrap());

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(!response.errors.is_empty());
    assert!(response.errors.iter().any(|e| e.message.contains(&format!("Failed to update user with id '{}'", fixed_uuid))));
}

#[tokio::test]
async fn test_update_user_db_error() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock input for updating a user
    let input = UpdateUserInput {
        id: fixed_uuid,
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: None,
    };

    // Mock the database to return an error during update
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: fixed_uuid,
            username: "original_user".to_owned(),
            email: "original@example.com".to_owned(),
            password: "hashed_password".to_owned(),
            first_name: "original".to_owned(),
            last_name: "user".to_owned(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }]]) // Simulate finding the user
        .append_exec_errors([DbErr::Custom("Update error".into())]) // Simulate an error during update
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            updateUser(input: {{
                id: "{}",
                username: "{}",
                email: "{}"
            }}) {{
                id
                username
                email
            }}
        }}"#, input.id, input.username.clone().unwrap(), input.email.clone().unwrap());

    let response = schema.execute(&query).await;

    // Assert the response
    assert!(!response.errors.is_empty());
    assert!(response.errors.iter().any(|e| e.message.contains("Failed to update user with id")));
}

#[tokio::test]
async fn test_delete_user_success() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate successful user deletion
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful deletion
            last_insert_id: 0,
        }])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            deleteUser(id: "{}")
        }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Debug print response for troubleshooting
    println!("Response: {:?}", response);

    // Assert the response
    assert!(response.errors.is_empty(), "Unexpected errors: {:?}", response.errors);
    let data = response.data.into_json().unwrap();

    // Assertions
    assert_eq!(data["deleteUser"], true, "User should be deleted successfully");
}

#[tokio::test]
async fn test_delete_user_not_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate user not found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 0, // Simulate no user found for deletion
            last_insert_id: 0,
        }])
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            deleteUser(id: "{}")
        }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Debug print response for troubleshooting
    println!("Response: {:?}", response);

    // Assert the response
    assert!(response.errors.is_empty(), "Unexpected errors: {:?}", response.errors);
    let data = response.data.into_json().unwrap();

    // Assertions
    assert_eq!(data["deleteUser"], false, "User should not be found for deletion");
}

#[tokio::test]
async fn test_delete_user_db_error() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate a database error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_errors([DbErr::Custom("Deletion error".into())]) // Simulate a database error
        .into_connection();

    let db = Arc::new(db);

    // Create a schema with the mock database in context
    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish();

    // Execute the mutation
    let query = format!(r#"
        mutation {{
            deleteUser(id: "{}")
        }}"#, fixed_uuid);

    let response = schema.execute(&query).await;

    // Debug print response for troubleshooting
    println!("Response: {:?}", response);

    // Assert the response
    assert!(!response.errors.is_empty(), "Expected errors but got none");
    let errors = response.errors;
    assert!(errors.iter().any(|e| e.message.contains(&format!("Failed to delete user with id '{}'", fixed_uuid))));
}
