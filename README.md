# 🦀 Rust Employee Directory API

This project is a high-performance, asynchronous REST API built with Rust. It serves as a modern replacement for a traditional Java/Spring Boot backend, utilizing a PostgreSQL database.

## 🎯 Rationale: Why Rust for Spring Boot Developers?

If you are coming from a Spring Boot background (like Chad Darby's courses), here is why this architecture was chosen:

| Feature | Spring Boot (Java) | Axum/SQLx (Rust) | Why it matters |
| --- | --- | --- | --- |
| **Runtime** | JVM (Heavyweight) | Native Binary (Lightweight) | Lower RAM usage (~20MB vs 300MB+). |
| **Concurrency** | Thread-per-request | **Tokio** (Async/Event-driven) | Handles thousands of concurrent connections with minimal overhead. |
| **Null Safety** | Optional / Runtime Checks | **`Option<T>`** | Eliminates `NullPointerExceptions` at compile-time. |
| **Database** | Hibernate / JPA (Runtime) | **SQLx** (Compile-time) | SQL queries are verified against the real DB during compilation. |
| **Startup** | Seconds (Slow) | Milliseconds (Instant) | Critical for containerized environments and rapid development. |

---

## 🏗 Key Components & Documentation

### **Web Framework: [Axum**](https://docs.rs/axum/latest/axum/)

* **Role**: Handles routing and HTTP requests.
* **Spring Equivalent**: `@RestController` and `RequestMapping`.
* **Note**: Axum is built on top of `tower` and `hyper`, making it part of the most robust ecosystem in Rust web dev.

### **Async Runtime: [Tokio**](https://tokio.rs/)

* **Role**: The underlying engine for asynchronous execution.
* **Spring Equivalent**: Spring WebFlux / Project Reactor.

### **Database Toolkit: [SQLx**](https://github.com/launchbadge/sqlx)

* **Role**: A "raw" SQL toolkit with compile-time safety.
* **Spring Equivalent**: JDBC Template / Spring Data JPA.
* **Feature**: It doesn't hide SQL from you, but ensures your SQL is correct before the app even runs.

### **Documentation: [Utoipa**](https://www.google.com/search?q=https://github.com/juhakivekas/utoipa)

* **Role**: Generates OpenAPI specifications.
* **Spring Equivalent**: SpringDoc / Swagger UI.

---

## 🛠 Project Structure & Logic

* **`src/main.rs`**: Setup of the `PgPool` (Connection Pool) and the Router. We use `State` to share the database connection, which is Rust's way of doing **Dependency Injection**.
* **`src/models.rs`**: Definitions of our data structures. We use `serde` for JSON serialization/deserialization (the equivalent of **Jackson** in Java).
* **`src/handlers.rs`**: Contains the logic for processing requests (the Service/Controller layer).

---

## 📝 Essential Commands

* **Build & Run**: `cargo run`
* **Check SQL Validity**: `cargo check` (This will contact your Podman DB to verify queries!)
* **Add Dependencies**: `cargo add <crate_name>`

---

## 🔗 Official Resources

* [The Rust Book](https://doc.rust-lang.org/book/) - The "Bible" of Rust.
* [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples) - Best practices for API design.
* [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/) - Deep dive into database handling.
