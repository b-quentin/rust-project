use async_graphql::{
    EmptyMutation,
    EmptySubscription,
    Schema
};
use sea_orm::{DatabaseBackend, DbErr, MockDatabase, MockExecResult};
use uuid::Uuid;
use crate::internal::api::user::{
    controllers::graphql::{user::{CreateUserInput, UpdateUserInput}, MutationRoot, QueryRoot},
    models::user
};

#[tokio::test]
async fn test_user_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database with a matching user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![user::Model {
                id: fixed_uuid,
                username: "test_user".to_owned(),
                email: "test@example.com".to_owned(),
                password: "hashed_password".to_owned(),
            }],
        ])
        .into_connection();

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
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

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
async fn test_create_user_success() {
    // Mock input for creating a user
    let input = CreateUserInput {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Fixed UUID for the new user
    let generated_uuid = Uuid::new_v4();

    // Mock the database to simulate successful user creation
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![user::Model {
            id: generated_uuid,
            username: input.username.clone(),
            email: input.email.clone(),
            password: "hashed_password".to_owned(),
        }]]) // Correctly simulate the returned user after creation
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful insertion
            last_insert_id: 0, // Not used but kept for structure
        }])
        .into_connection();

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
            }}) {{
                id
                username
                email
            }}
        }}"#, input.username, input.email, input.password);

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
    };

    // Mock the database to return an error during user creation
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // No prior users in the system
        .append_exec_errors([DbErr::Custom("Insertion error".into())]) // Simulate a database error
        .into_connection();

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
            }}) {{
                id
                username
                email
            }}
        }}"#, input.username, input.email, input.password);

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
        .append_query_results([vec![user::Model {
            id: fixed_uuid,
            username: "original_user".to_owned(),
            email: "original@example.com".to_owned(),
            password: "hashed_password".to_owned(),
        }]]) // Simulate finding the user
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful update
            last_insert_id: 0,
        }])
        .append_query_results([vec![user::Model {
            id: fixed_uuid,
            username: input.username.clone().unwrap(),
            email: input.email.clone().unwrap(),
            password: "hashed_password".to_owned(),
        }]]) // Simulate returning the updated user
        .into_connection();

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
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

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
        .append_query_results([vec![user::Model {
            id: fixed_uuid,
            username: "original_user".to_owned(),
            email: "original@example.com".to_owned(),
            password: "hashed_password".to_owned(),
        }]]) // Simulate finding the user
        .append_exec_errors([DbErr::Custom("Update error".into())]) // Simulate an error during update
        .into_connection();

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
