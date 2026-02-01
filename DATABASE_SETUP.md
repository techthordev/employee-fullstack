# Database setup (Podman)

This project uses a PostgreSQL database running in a Podman container.

---

## Important note about Podman Compose

`podman compose` is intended to **create and manage containers**, not to attach to an already running one.

- Run `podman compose up` **only once** to create the database container.
- If a PostgreSQL container is already running, **do not run `podman compose up` again**.
- Running `podman compose up` while the database container is already running may cause port conflicts (`address already in use`).

---

## Recommended workflow

1. Start the database (initial setup only):

```bash
podman compose up -d
````

2. Subsequent runs, just start/stop the existing container:

```bash
podman start <container_name>
podman stop <container_name>
```

3. To remove the database completely (including volumes):

```bash
podman compose down -v
```

---

## Database connection

* Host: `localhost`
* Port: `5433` (or as defined in `.env`)
* Database: `student_tracker` (or `employee_directory`)
* User: `postgres`
* Password: as defined in the environment variables

You can inspect and manage the database using tools such as **DBeaver**.
