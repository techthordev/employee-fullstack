# Employee Directory Database

## Overview

PostgreSQL database setup for the Spring Boot Employee Directory application.

**Database:** PostgreSQL 18  
**Container:** Podman  
**Port:** 5434 (host) → 5432 (container)

---

## Quick Start

```bash
# Start database (first time)
podman compose up -d

# Stop database
podman compose down

# View logs
podman compose logs -f

# Connect to database
podman exec -it employee-directory-db psql -U springstudent -d employee_directory
````

---

## Configuration Files

### Environment Variables

`.env` in project root:

```properties
POSTGRES_PASSWORD=postgres
POSTGRES_PORT=5434
```

**Note:** `.env` is git-ignored and contains local settings only.

---

## Database Schema

### Employee Table

| Column     | Type        | Description                  |
| ---------- | ----------- | ---------------------------- |
| id         | SERIAL      | Primary key (auto-increment) |
| first_name | VARCHAR(45) | Employee first name          |
| last_name  | VARCHAR(45) | Employee last name           |
| email      | VARCHAR(45) | Employee email               |

### Auto-Increment Configuration

```sql
ALTER SEQUENCE employee_id_seq RESTART WITH 1000;
SELECT last_value FROM employee_id_seq;
TRUNCATE TABLE employee RESTART IDENTITY;
```

---

## Initialization Scripts

`database/init/`:

1. **01-create-user.sql** - Creates `springstudent`
2. **02-student-tracker.sql** - Legacy / optional
3. **employee-directory.sql** - Employee schema and data

To re-run initialization:

```bash
podman compose down -v
podman compose up -d
```

---

## DBeaver Connection

### Application User (`springstudent`)

| Setting  | Value              |
| -------- | ------------------ |
| Host     | localhost          |
| Port     | 5434               |
| Database | employee_directory |
| Username | springstudent      |
| Password | springstudent      |

---

## Spring Boot Configuration

`src/main/resources/application.properties`:

```properties
spring.datasource.url=jdbc:postgresql://localhost:5434/employee_directory
spring.datasource.username=springstudent
spring.datasource.password=springstudent
spring.datasource.driver-class-name=org.postgresql.Driver

spring.jpa.database-platform=org.hibernate.dialect.PostgreSQLDialect
spring.jpa.hibernate.ddl-auto=update
spring.jpa.show-sql=true
```

---

## Common Commands

```bash
podman exec -it employee-directory-db psql -U springstudent -d employee_directory
\dt
\d employee
\q
```

### Troubleshooting

```sql
GRANT ALL ON SCHEMA public TO springstudent;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO springstudent;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO springstudent;
```

### Clean Slate

```bash
podman compose down -v
podman compose up -d
```
