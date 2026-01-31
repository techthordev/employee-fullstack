// Declare our sub-modules and make them public.
// This allows main.rs and other parts of the app to use them.
pub mod models;
pub mod handlers;
pub mod db;

use axum::{routing::get, Router};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// This structure defines our OpenAPI (Swagger) documentation.
// It lists all the endpoints and the data models we use.
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::employee_handler::list_employees,
        handlers::employee_handler::create_employee,
        handlers::employee_handler::get_employee_by_id,
        handlers::employee_handler::update_employee,
        handlers::employee_handler::delete_employee
    ),
    components(schemas(models::Employee, models::CreateEmployee))
)]
struct ApiDoc;

/// This function builds the entire application.
/// It sets up the routes (URLs) and attaches the database pool.
pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        // Simple health check route
        .route("/", get(|| async { "Employee API is running" }))
        
        // Define routes for /api/employees
        .route(
            "/api/employees", 
            // We combine GET and POST for the same URL here.
            get(handlers::employee_handler::list_employees)
            .post(handlers::employee_handler::create_employee)
        )
        // Define route for one employee by id /api/employees/{id}
        .route(
            "/api/employees/{id}",
            get(handlers::employee_handler::get_employee_by_id)
            .put(handlers::employee_handler::update_employee)
            .delete(handlers::employee_handler::delete_employee)
        )
        
        // Plug in the Swagger UI so we can test the API in the browser.
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        // Share the database pool with all our handlers.
        .with_state(pool)
}