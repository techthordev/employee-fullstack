use axum::{extract::{State, Path}, Json};
use axum::http::StatusCode;
use sqlx::PgPool;
use crate::models::{CreateEmployee, Employee};

/// This tells Swagger (utoipa) how to document the GET request.
#[utoipa::path(
    get,
    path = "/api/employees",
    responses(
        (status = 200, description = "List all employees", body = [Employee])
    )
)]
pub async fn list_employees(
    // We "extract" the database pool that we stored in the App State earlier.
    State(pool): State<PgPool>,
) -> Json<Vec<Employee>> {
    // We run a SQL query to get all rows from the 'employee' table.
    // The 'query_as!' macro automatically maps the columns to our Employee struct.
    let employees = sqlx::query_as!(
        Employee,
        "SELECT id, first_name, last_name, email FROM employee ORDER BY id"
    )
    .fetch_all(&pool) // Get all results from the database.
    .await            // Wait for the database to finish.
    .unwrap_or_default(); // If something goes wrong, return an empty list instead of crashing.
    
    // Wrap the list in Json() so Axum sends it as a JSON response to the browser.
    Json(employees)
}

/// This tells Swagger (utoipa) how to document the POST request.
#[utoipa::path(
    post,
    path = "/api/employees",
    request_body = CreateEmployee,
    responses(
        (status = 201, description = "Employee created successfully", body = Employee)
    )
)]
pub async fn create_employee(
    // Get the database pool from the state.
    State(pool): State<PgPool>,
    // Automatically convert the incoming JSON body into our CreateEmployee struct.
    Json(payload): Json<CreateEmployee>,
) -> Json<Employee> {
    // We insert a new row into the database using the data from 'payload'.
    // The RETURNING clause gives us back the new row (including the ID created by the DB).
    let employee = sqlx::query_as!(
        Employee,
        "INSERT INTO employee (first_name, last_name, email) 
        VALUES ($1, $2, $3) 
        RETURNING id, first_name as \"first_name?\", last_name as \"last_name?\", email as \"email?\"",
        payload.first_name,
        payload.last_name,
        payload.email
    )
    .fetch_one(&pool) // We expect exactly one record back (the one we just created).
    .await            // Wait for the database.
    .expect("Failed to insert employee!"); // Crash if the insert fails (we will fix this with error handling later).
    
    // Return the newly created employee as JSON with a success status.
    Json(employee)
}

/// Swagger documentation for getting a single employee by ID
#[utoipa::path(
    get,
    path = "/api/employees/{id}",
    responses(
        (status = 200, description = "Employee found", body = Employee),
        (status = 404, description = "Employee not found")
    ),
    params(
        ("id" = i32, Path, description = "Employee database id")
    )
)]
pub async fn get_employee_by_id(
    // We get the database connection from Axum's state.
    State(pool): State<PgPool>,
    // We extract the 'id' from the URL path. 
    // Example: In /api/employees/10, the id will be 10.
    Path(id): Path<i32>,
) -> Json<Option<Employee>> {
    // We search for the employee with this specific ID.
    let employee = sqlx::query_as!(
        Employee,
        "SELECT id, first_name, last_name, email FROM employee WHERE id = $1",
        id
    )
    .fetch_optional(&pool) // Returns Some(employee) if found, or None if not found.
    .await
    .unwrap_or(None); // If a database error occurs, we return None.

    // Wrap the result in Json so the client receives it in JSON format.
    Json(employee)
}

/// This handler updates an existing employee's data.
#[utoipa::path(
    put,
    path = "/api/employees/{id}",
    request_body = CreateEmployee,
    responses(
        (status = 200, description = "Employee updated successfully", body = Employee),
        (status = 404, description = "Employee not found")
    ),
    params(
        ("id" = i32, Path, description = "The ID of the employee to update")
    )
)]
pub async fn update_employee(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateEmployee>,
) -> Result<Json<Employee>, StatusCode> {
    // We update the record and use RETURNING to get the updated row back.
    let result = sqlx::query_as!(
        Employee,
        "UPDATE employee 
         SET first_name = $1, last_name = $2, email = $3 
         WHERE id = $4 
         RETURNING id, first_name as \"first_name?\", last_name as \"last_name?\", email as \"email?\"",
        payload.first_name,
        payload.last_name,
        payload.email,
        id
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(employee)) => Ok(Json(employee)), // Found and updated.
        Ok(None) => Err(StatusCode::NOT_FOUND),   // ID didn't exist.
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// This handler removes an employee from the database.
#[utoipa::path(
    delete,
    path = "/api/employees/{id}",
    responses(
        (status = 204, description = "Employee deleted successfully"),
        (status = 404, description = "Employee not found")
    ),
    params(
        ("id" = i32, Path, description = "The ID of the employee to delete")
    )
)]
pub async fn delete_employee(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    // We try to delete the row where the ID matches.
    let result = sqlx::query!("DELETE FROM employee WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        // If the database query finished successfully:
        Ok(res) => {
            // We check if actually anyone was deleted.
            if res.rows_affected() > 0 {
                StatusCode::NO_CONTENT // 204: Success, but no data to send back.
            } else {
                StatusCode::NOT_FOUND // 404: The ID didn't exist.
            }
        }
        // If the database had an error:
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500: Something went wrong on the server.
    }
}