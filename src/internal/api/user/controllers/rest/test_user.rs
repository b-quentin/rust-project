use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use sea_orm::{DatabaseBackend, DbErr, MockDatabase, MockExecResult};
use uuid::Uuid;
use crate::internal::api::user::controllers::rest::{CreateUserInput, UpdateUserInput};
use crate::internal::api::user::models::user;
use crate::internal::api::user::controllers::rest::user::get_user;
use crate::internal::api::user::routes::init_user_routes;

#[actix_rt::test]
async fn test_get_user_success() {
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

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route("/user/{id}", web::get().to(get_user)),
    )
    .await;

    // Execute a test request to the get_user endpoint
    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", fixed_uuid))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status and body
    assert_eq!(resp.status(), 200);
    let body: user::Model = test::read_body_json(resp).await;
    assert_eq!(body.id, fixed_uuid);
    assert_eq!(body.username, "test_user");
    assert_eq!(body.email, "test@example.com");
}

#[actix_rt::test]
async fn test_get_user_not_found() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to simulate no user found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Simulate empty result
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route("/user/{id}", web::get().to(get_user)),
    )
    .await;

    // Execute a test request to the get_user endpoint
    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", fixed_uuid))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_get_user_db_error() {
    // Fixed UUID for testing
    let fixed_uuid = Uuid::parse_str("51c84da0-6fbe-4db2-81fe-385a38d29353").unwrap();

    // Mock the database to return an error during user retrieval
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors([DbErr::Custom("Database error".into())]) // Simulate a database error
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route("/user/{id}", web::get().to(get_user)),
    )
    .await;

    // Execute a test request to the get_user endpoint
    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", fixed_uuid))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), 500);
}

#[actix_rt::test]
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
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful insertion
            last_insert_id: 0, // Not used but included for structure
        }])
        .append_query_results([vec![user::Model {
            id: generated_uuid,
            username: input.username.clone(),
            email: input.email.clone(),
            password: "hashed_password".to_owned(),
        }]]) // Simulate returning the newly created user
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function for setting routes
    )
    .await;

    // Execute a test request to the create_user endpoint
    let req = test::TestRequest::post()
        .uri("/api/user/")  // Correct URI according to your scope
        .set_json(&input)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status and body
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: user::Model = test::read_body_json(resp).await;
    assert_eq!(body.id, generated_uuid);
    assert_eq!(body.username, "test_user");
    assert_eq!(body.email, "test@example.com");
}

#[actix_rt::test]
async fn test_create_user_db_error() {
    // Mock input for creating a user
    let input = CreateUserInput {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Mock the database to return an error during user creation
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_errors([DbErr::Custom("Insertion error".into())]) // Simulate a database error
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function for setting routes
    )
    .await;

    // Execute a test request to the create_user endpoint
    let req = test::TestRequest::post()
        .uri("/api/user/")  // Correct URI according to your scope
        .set_json(&input)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn test_update_user_success() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock input for updating a user
    let input = UpdateUserInput {
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: Some("new_password".to_string()),
    };

    // Mock the database to simulate successful user update
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![user::Model {
            id: user_id,
            username: "old_user".to_owned(),
            email: "old@example.com".to_owned(),
            password: "old_password".to_owned(),
        }]]) // Simulate finding the user
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful update
            last_insert_id: 0,
        }])
        .append_query_results([vec![user::Model {
            id: user_id,
            username: input.username.clone().unwrap(),
            email: input.email.clone().unwrap(),
            password: "hashed_new_password".to_owned(),
        }]]) // Simulate returning the updated user
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the update_user endpoint
    let req = test::TestRequest::put()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .set_json(&input)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status and body
    assert_eq!(resp.status(), StatusCode::OK);
    let body: user::Model = test::read_body_json(resp).await;
    assert_eq!(body.id, user_id);
    assert_eq!(body.username, "updated_user");
    assert_eq!(body.email, "updated@example.com");
}

#[actix_rt::test]
async fn test_update_user_not_found() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock input for updating a user
    let input = UpdateUserInput {
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: Some("new_password".to_string()),
    };

    // Mock the database to simulate user not found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<user::Model, Vec<user::Model>, _>([vec![]]) // Simulate empty result
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the update_user endpoint
    let req = test::TestRequest::put()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .set_json(&input)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn test_update_user_db_error() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock input for updating a user
    let input = UpdateUserInput {
        username: Some("updated_user".to_string()),
        email: Some("updated@example.com".to_string()),
        password: Some("new_password".to_string()),
    };

    // Mock the database to return an error during update
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![user::Model {
            id: user_id,
            username: "old_user".to_owned(),
            email: "old@example.com".to_owned(),
            password: "old_password".to_owned(),
        }]]) // Simulate finding the user
        .append_exec_errors([DbErr::Custom("Update error".into())]) // Simulate an error during update
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the update_user endpoint
    let req = test::TestRequest::put()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .set_json(&input)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn test_delete_user_success() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock the database to simulate successful user deletion
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 1, // Simulate successful deletion
            last_insert_id: 0,
        }])
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the delete_user endpoint
    let req = test::TestRequest::delete()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_delete_user_not_found() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock the database to simulate user not found
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results([MockExecResult {
            rows_affected: 0, // Simulate no user found for deletion
            last_insert_id: 0,
        }])
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the delete_user endpoint
    let req = test::TestRequest::delete()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_delete_user_db_error() {
    // Fixed UUID for testing
    let user_id = Uuid::new_v4();

    // Mock the database to return an error during deletion
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_errors([DbErr::Custom("Deletion error".into())]) // Simulate a database error
        .into_connection();

    // Set up the Actix Web app with the mock database
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_user_routes)  // Use the correct init function to set up routes
    )
    .await;

    // Execute a test request to the delete_user endpoint
    let req = test::TestRequest::delete()
        .uri(&format!("/api/user/{}", user_id)) // Correct URI according to your scope
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
