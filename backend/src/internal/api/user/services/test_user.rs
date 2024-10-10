use crate::internal::api::user::services::user::*;
use crate::internal::api::user::models::user;
use sea_orm::{
    DatabaseBackend, DbErr, MockDatabase, MockExecResult
};
use uuid::Uuid;

#[tokio::test]
async fn test_create_user() -> Result<(), DbErr> {
    // Configure the mock database
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![user::Model {
                id: Uuid::new_v4(),
                username: "test_user".to_owned(),
                firstname: "test".to_owned(),
                lastname: "user".to_owned(),
                email: "test@example.com".to_owned(),
                password: "hashed_password".to_owned(),
            }],
        ])
        .into_connection();

    // Create the user using the mock database
    let result = UserServiceImpl::create_user(
        &db,
        "test_user".to_string(),
        "test".to_string(),
        "user".to_string(),
        "test@example.com".to_string(),
        "password".to_string()
    ).await;

    // Ensure that user creation was successful
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, "test@example.com");

    Ok(())
}

#[tokio::test]
async fn test_create_user_db_error() -> Result<(), DbErr> {
    // Configure the mock database to return an error on insert
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Insertion error".into())]) // Simulate an insertion error
        .into_connection();

    // Act: Call the create_user function with the mock database
    let result = UserServiceImpl::create_user(
        &db,
        "test_user".to_string(),
        "test".to_string(),
        "user".to_string(),
        "test@example.com".to_string(),
        "password".to_string(),
    ).await;

    // Assert: Ensure that the function returns the correct error
    assert!(result.is_err());
    if let Err(DbErr::Custom(e)) = result {
        assert_eq!(e, "Insertion error");
    } else {
        panic!("Expected custom insertion error");
    }

    Ok(())
}

#[tokio::test]
async fn test_get_user_found() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Configure the mock database with a matching user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![user::Model {
                id: fixed_uuid,
                username: "test_user".to_owned(),
                firstname: "test".to_owned(),
                lastname: "user".to_owned(),
                email: "test@example.com".to_owned(),
                password: "hashed_password".to_owned(),
            }],
        ])
        .into_connection();

    // Call the get_user function with the mock database
    let result = UserServiceImpl::get_user(&db, fixed_uuid).await;

    // Ensure that user is found
    assert!(result.is_ok());
    let user = result.unwrap();
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.id, fixed_uuid);
    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, "test@example.com");

    Ok(())
}

#[tokio::test]
async fn test_get_user_not_found() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    // Call the get_user function with the mock database
    let result = UserServiceImpl::get_user(&db, fixed_uuid).await;

    // Ensure that no user is found
    assert!(result.is_ok());
    let user = result.unwrap();
    assert!(user.is_none());

    Ok(())
}

#[tokio::test]
async fn test_get_user_db_error() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Configure the mock database to return an error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Database error".into())]) // Simulate an error
        .into_connection();

    // Call the get_user function with the mock database
    let result = UserServiceImpl::get_user(&db, fixed_uuid).await;

    // Ensure that an error is returned
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_get_all_users_success() {
    // Mock UUIDs for testing
    let uuid1 = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();
    let uuid2 = Uuid::parse_str("f30c1f8f-55c8-4ad5-b3e8-4f4530a73a58").unwrap();

    // Mock the database with multiple users
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![
            user::Model {
                id: uuid1,
                username: "test_user1".to_owned(),
                firstname: "test".to_owned(),
                lastname: "user".to_owned(),
                email: "test1@example.com".to_owned(),
                password: "hashed_password1".to_owned(),
            },
            user::Model {
                id: uuid2,
                username: "test_user2".to_owned(),
                firstname: "test".to_owned(),
                lastname: "user".to_owned(),
                email: "test2@example.com".to_owned(),
                password: "hashed_password2".to_owned(),
            },
        ]])
        .into_connection();

    // Call the function to be tested
    let result = UserServiceImpl::get_all_users(&db).await;

    // Assert the result
    assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result);
    let users = result.unwrap();
    assert_eq!(users.len(), 2, "Expected 2 users but found {}", users.len());

    // Assertions for the first user
    assert_eq!(users[0].id, uuid1);
    assert_eq!(users[0].username, "test_user1");
    assert_eq!(users[0].email, "test1@example.com");

    // Assertions for the second user
    assert_eq!(users[1].id, uuid2);
    assert_eq!(users[1].username, "test_user2");
    assert_eq!(users[1].email, "test2@example.com");
}

#[tokio::test]
async fn test_get_all_users_db_error() {
    // Mock the database to simulate a database error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Simulated database error".into())]) // Simulate a DB error
        .into_connection();

    // Call the function to be tested
    let result = UserServiceImpl::get_all_users(&db).await;

    // Assert the result
    assert!(result.is_err(), "Expected Err but got Ok: {:?}", result);
    let error = result.err().unwrap();

    // Assertions to confirm the error type and message
    match error {
        DbErr::Custom(msg) => assert_eq!(msg, "Simulated database error"),
        _ => panic!("Expected Custom error but got {:?}", error),
    }
}

#[tokio::test]
async fn test_find_user_by_email_success() {
    // Mock UUID for testing
    let uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database with a user matching the email
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![user::Model {
            id: uuid,
            username: "test_user".to_owned(),
            firstname: "test".to_owned(),
            lastname: "user".to_owned(),
            email: "test@example.com".to_owned(),
            password: "hashed_password".to_owned(),
        }]])
        .into_connection();

    // Call the function to be tested
    let result = UserServiceImpl::find_user_by_email(&db, "test@example.com".to_string()).await;

    // Assert the result
    assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result);
    let user = result.unwrap();

    // Assertions for the found user
    assert!(user.is_some(), "Expected Some(user) but got None");
    let user = user.unwrap();
    assert_eq!(user.id, uuid);
    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_find_user_by_email_not_found() {
    // Mock the database to simulate no user found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    // Call the function to be tested
    let result = UserServiceImpl::find_user_by_email(&db, "nonexistent@example.com".to_string()).await;

    // Assert the result
    assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result);
    let user = result.unwrap();

    // Assertions to confirm no user is returned
    assert!(user.is_none(), "Expected None but got Some(user)");
}

#[tokio::test]
async fn test_find_user_by_email_db_error() {
    // Mock the database to simulate a database error
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Simulated database error".into())]) // Simulate a DB error
        .into_connection();

    // Call the function to be tested
    let result = UserServiceImpl::find_user_by_email(&db, "error@example.com".to_string()).await;

    // Assert the result
    assert!(result.is_err(), "Expected Err but got Ok: {:?}", result);
    let error = result.err().unwrap();

    // Assertions to confirm the error type and message
    match error {
        DbErr::Custom(msg) => assert_eq!(msg, "Simulated database error"),
        _ => panic!("Expected Custom error but got {:?}", error),
    }
}

#[tokio::test]
async fn test_update_user_not_found() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to return no user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Correctly specify the type for empty results
        .into_connection();

    // Act: Call the update_user function
    let result = UserServiceImpl::update_user(
        &db,
        fixed_uuid,
        Some("new_username".to_string()),
        Some("new_email@example.com".to_string()),
        None,
    ).await;

    // Assert: Ensure the function returns a RecordNotFound error
    assert!(result.is_err());
    if let Err(DbErr::RecordNotFound(_)) = result {
        // Test passes if the error is RecordNotFound
    } else {
        panic!("Expected RecordNotFound error");
    }

    Ok(())
}

#[tokio::test]
async fn test_update_user_success() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the initial state of the user in the database
    let initial_user = user::Model {
        id: fixed_uuid,
        username: "old_username".to_string(),
        firstname: "old_first".to_string(),
        lastname: "old_last".to_string(),
        email: "old_email@example.com".to_string(),
        password: "old_password_hash".to_string(),
    };

    // Mock the database with the initial user and expected updated user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![initial_user.clone()], // Initial fetch
            vec![user::Model { // Expected updated user
                id: fixed_uuid,
                username: "new_username".to_owned(),
                firstname: "old_first".to_owned(),
                lastname: "old_last".to_owned(),
                email: "new_email@example.com".to_owned(),
                password: "old_password_hash".to_owned(), // Assuming password isn't updated in this test
            }],
        ])
        .into_connection();

    // Act: Call the update_user function with updated values
    let result = UserServiceImpl::update_user(
        &db,
        fixed_uuid,
        Some("new_username".to_string()),
        Some("new_email@example.com".to_string()),
        None, // No password update
    ).await;

    // Assert: Ensure the update was successful
    assert!(result.is_ok());
    let updated_user = result.unwrap();
    assert_eq!(updated_user.username, "new_username");
    assert_eq!(updated_user.email, "new_email@example.com");
    assert_eq!(updated_user.password, "old_password_hash");

    Ok(())
}

#[tokio::test]
async fn test_update_user_db_error() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to return an error when fetching the user
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Database error".into())]) // Simulate DB error
        .into_connection();

    // Act: Call the update_user function
    let result = UserServiceImpl::update_user(
        &db,
        fixed_uuid,
        Some("new_username".to_string()),
        Some("new_email@example.com".to_string()),
        None,
    ).await;

    // Assert: Ensure the function returns the correct error
    assert!(result.is_err());
    if let Err(DbErr::Custom(e)) = result {
        assert_eq!(e, "Database error");
    } else {
        panic!("Expected custom database error");
    }

    Ok(())
}

#[tokio::test]
async fn test_delete_user_success() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate successful deletion (1 row affected)
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate that one row was successfully deleted
            last_insert_id: 0,
        }])
        .into_connection();

    // Act: Call the delete_user function with the mock database
    let result = UserServiceImpl::delete_user(&db, fixed_uuid).await;

    // Assert: Ensure the deletion was successful
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[tokio::test]
async fn test_delete_user_not_found() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate that no user was found for deletion (0 rows affected)
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 0, // Simulate no rows were deleted
            last_insert_id: 0,
        }])
        .into_connection();

    // Act: Call the delete_user function with the mock database
    let result = UserServiceImpl::delete_user(&db, fixed_uuid).await;

    // Assert: Ensure that the result is false indicating no user was found to delete
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    Ok(())
}

#[tokio::test]
async fn test_delete_user_db_error() -> Result<(), DbErr> {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to return an error during deletion
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_errors([DbErr::Custom("Deletion error".into())]) // Correctly simulate a database error during execution
        .into_connection();

    // Act: Call the delete_user function with the mock database
    let result = UserServiceImpl::delete_user(&db, fixed_uuid).await;

    // Assert: Ensure that an error is returned
    assert!(result.is_err());
    if let Err(DbErr::Custom(e)) = result {
        assert_eq!(e, "Deletion error");
    } else {
        panic!("Expected custom deletion error");
    }

    Ok(())
}
