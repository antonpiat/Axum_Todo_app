# Axum Todo App

> **To do, or not to do, that is the question.**

A full-stack todo application with a **Rust (Axum)** backend, **Next.js** frontend, and **PostgreSQL** database. Users can sign up, log in, and manage their personal todo lists with create, update, and delete operations.

---

## Features

- **User authentication** — Register, login, logout, and session checks via HTTP cookies
- **Secure passwords** — Argon2 hashing before storage
- **Per-user todos** — Each user sees and manages only their own tasks
- **REST API** — CRUD endpoints for todos, protected by auth middleware
- **Interactive API docs** — Swagger UI powered by Utoipa
- **Modern UI** — Next.js App Router, Tailwind CSS, and Zustand state management

---

## Tech Stack

| Layer      | Technologies |
| ---------- | ------------ |
| Backend    | [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx), [Tokio](https://tokio.rs), [Utoipa](https://github.com/juhaku/utoipa), Argon2 |
| Frontend   | [Next.js 16](https://nextjs.org), [React 19](https://react.dev), [Tailwind CSS 4](https://tailwindcss.com), [Zustand](https://github.com/pmndrs/zustand), Lucide icons |
| Database   | [PostgreSQL 16](https://www.postgresql.org) |
| Tooling    | Docker Compose, pnpm, Cargo |

---

## Architecture

```mermaid
flowchart LR
    subgraph client [Client]
        UI[Next.js Frontend<br/>:3000]
    end

    subgraph server [Backend]
        API[Axum API<br/>:8080]
        SW[Swagger UI<br/>/swagger-ui]
    end

    subgraph data [Data]
        DB[(PostgreSQL<br/>:5432)]
    end

    UI -->|REST + cookies| API
    API --> DB
    SW --> API
```

In production-style runs, the Axum server can also serve the static Next.js export from `frontend/out`.

---

## Project Structure

```
Axum_Todo_app/
├── backend/
│   └── src/
│       ├── main.rs           # Server entry, routing, CORS, Swagger
│       ├── user/             # Auth handlers, models, password hashing
│       └── todo/             # Todo CRUD handlers and models
├── frontend/
│   ├── app/                  # Next.js pages (home, login, signup)
│   ├── components/           # TodoList, TodoCard, AuthForm
│   └── store/                # Zustand user & todo state
├── docker-compose.yml        # PostgreSQL service
├── init.sql                  # Database schema
└── README.md
```

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (latest stable)
- [Node.js](https://nodejs.org) 20+
- [pnpm](https://pnpm.io)
- [Docker](https://www.docker.com) & Docker Compose

### 1. Clone the repository

```bash
git clone https://github.com/your-username/Axum_Todo_app.git
cd Axum_Todo_app
```

### 2. Configure environment variables

Create a `.env` file in the project root for Docker Compose:

```env
POSTGRES_USER=todo_user
POSTGRES_PASSWORD=your_secure_password
POSTGRES_DB=todo_db
```

Create a `backend/.env` file for the Rust server:

```env
SERVER_ADDRESS=127.0.0.1:8080
DB_URL=postgres://todo_user:your_secure_password@127.0.0.1:5432/todo_db
```

> Use the same credentials in both files so the backend can connect to the Dockerized database.

### 3. Start the database

```bash
docker compose up -d
```

This starts PostgreSQL and runs `init.sql` to create the `users` and `todos` tables.

### 4. Run the backend

```bash
cd backend
cargo run
```

The API listens on **http://127.0.0.1:8080**.

### 5. Run the frontend

In a separate terminal:

```bash
cd frontend
pnpm install
pnpm dev
```

Open **http://127.0.0.1:3000** in your browser.

---

## API Reference

### Authentication

| Method | Endpoint       | Description                          |
| ------ | -------------- | ------------------------------------ |
| `POST` | `/users`       | Register a new user                  |
| `POST` | `/login`       | Log in (sets `session` cookie)       |
| `POST` | `/logout`      | Log out (clears session cookie)      |
| `GET`  | `/check_auth`  | Verify current session               |

**Request body** (register / login):

```json
{
  "username": "johndoe",
  "password": "secret123"
}
```

### Todos

All todo routes require a valid session cookie.

| Method   | Endpoint            | Description                    |
| -------- | ------------------- | ------------------------------ |
| `GET`    | `/todo?username=`   | List todos for a user          |
| `POST`   | `/todo`             | Create a new todo              |
| `PUT`    | `/todo/{todo_id}`   | Update description or status   |
| `DELETE` | `/todo/{todo_id}`   | Delete a todo                  |

**Create todo body:**

```json
{
  "username": "johndoe",
  "description": "Buy groceries"
}
```

**Update todo body:**

```json
{
  "description": "Buy groceries and cook",
  "status": true
}
```

### Swagger UI

Interactive API documentation is available at:

**http://127.0.0.1:8080/swagger-ui**

---

## Database Schema

```sql
-- users
id          SERIAL PRIMARY KEY
username    TEXT NOT NULL UNIQUE
password    TEXT NOT NULL          -- Argon2 hash

-- todos
todo_id     TEXT PRIMARY KEY       -- UUID
username    TEXT NOT NULL
description TEXT NOT NULL
status      BOOLEAN DEFAULT false  -- completion flag
```

---

## Development

### Backend

```bash
cd backend
cargo run          # start server
cargo build        # compile only
cargo test         # run tests (if added)
```

### Frontend

```bash
cd frontend
pnpm dev           # dev server on :3000
pnpm build         # production build
pnpm lint          # ESLint
```

To serve the frontend from Axum, export the static build first:

```bash
cd frontend && pnpm build
cd ../backend && cargo run
```

The backend falls back to `frontend/out` for static assets.

---

## License

This project is open source. Add your preferred license here.
