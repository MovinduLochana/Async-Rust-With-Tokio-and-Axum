# Async-Rust-With-Tokio-and-Axum 🚀

This project demonstrates building asynchronous web applications in Rust using the Tokio and Axum frameworks, featuring robust error handling, request logging, and authentication mechanisms.

## Description 📝

This Rust project showcases the development of a web service using `Axum` and `Tokio`. It provides a foundation for building asynchronous APIs with features like user authentication, CRUD operations for tickets, and comprehensive request logging. The project is structured to separate concerns, with dedicated modules for context management, error handling, logging, and web routing.

## Table of Contents 🧭

- [Project Title & Badges](#async-rust-with-tokio-and-axum-🚀)
- [Description](#description-📝)
- [Table of Contents](#table-of-contents-🧭)
- [Features](#features-🌟)
- [Tech Stack](#tech-stack-💻)
- [Installation](#installation-install)
- [Usage](#usage-how-to-use-💡)
- [Project Structure](#project-structure-📂)
- [API Reference](#api-reference-🔗)
- [Important Links](#important-links-🔗)
- [Footer](#footer-®️)

## Features 🌟

- **Asynchronous Web Server:** Built with `Axum` and `Tokio` for efficient, non-blocking I/O operations.
- **CRUD Operations:** Implements Create, Read, Update (implicitly through delete and create), and Delete operations for tickets.
- **Authentication:** Includes middleware for context resolution and requiring authentication for specific routes.
- **Request Logging:** Logs details of incoming requests, including user ID, path, method, and any errors.
- **Error Handling:** Custom error types and middleware for mapping server errors to client-friendly responses.
- **Cookie Management:** Utilizes `tower-cookies` for handling authentication tokens.
- **State Management:** Uses `ModelController` with `Arc<Mutex<...>>` for managing in-memory data.
- **Static File Serving:** Supports serving static files from the `./` directory.

## Tech Stack 💻

- **Language:** Rust
- **Web Framework:** Axum
- **Async Runtime:** Tokio
- **Dependencies:** `tokio`, `axum`, `tower-http`, `tower-cookies`, `uuid`, `serde`, `serde_json`, `serde_with`, `strum_macros`, `lazy-regex`

## Installation 🛠️

1.  **Prerequisites:**
    *   Ensure you have Rust and Cargo installed.
    *   You can install Rust via [rustup](https://rustup.rs/).

2.  **Clone the Repository:**
    ```bash
    git clone https://github.com/MovinduLochana/Async-Rust-With-Tokio-and-Axum.git
    cd Async-Rust-With-Tokio-and-Axum
    ```

3.  **Build the Project:**
    ```bash
    cargo build
    ```

4.  **Run the Project:**
    ```bash
    cargo run
    ```
    The server will start on `http://localhost:8080`.

## Usage: How to use 💡

This project serves as a backend API. You can interact with it using tools like `curl` or Postman.

### API Endpoints:

1.  **Root Endpoint:**
    *   `GET /`
    *   Responds with: `Hello, world!`

2.  **User Creation (Example):**
    *   `POST /users`
    *   **Request Body:**
        ```json
        {
          "name": "John Doe"
        }
        ```
    *   **Response:**
        ```json
        {
          "id": 1000,
          "name": "John Doe"
        }
        ```

3.  **Parameter Test:**
    *   `GET /params?name=Alice`
    *   Responds with HTML: `<h1>Hello, Alice!</h1>`

4.  **Path Parameter Test:**
    *   `GET /path_test/some/path`
    *   Responds with HTML: `<h1>some/path</h1>`

5.  **Login:**
    *   `POST /api/login`
    *   **Request Body:**
        ```json
        {
          "username": "movindu",
          "password": "mypass"
        }
        ```
    *   **Response (Success):**
        ```json
        {
          "result": {
            "success": true
          }
        }
        ```
        Sets an `auth-token` cookie.
    *   **Response (Failure):**
        Returns a `403 Forbidden` status with an error message if credentials are invalid.

6.  **Ticket Operations (Requires Authentication):**
    These endpoints are protected and require a valid `auth-token` cookie.

    *   **Create Ticket:**
        *   `POST /api/tickets`
        *   **Request Body:**
            ```json
            {
              "title": "My First Ticket"
            }
            ```
        *   **Response:** Returns the created ticket with an ID.

    *   **List Tickets:**
        *   `GET /api/tickets`
        *   **Response:** Returns a list of all tickets.

    *   **Delete Ticket:**
        *   `DELETE /api/tickets/{id}` (e.g., `DELETE /api/tickets/0`)
        *   **Response:** Returns the deleted ticket.

7.  **Static File Serving:**
    *   `GET /static_files/...`
    *   Serves files from the root directory of the project.

### Example `curl` Commands:

**Login:**
```bash
curl -X POST \
  http://localhost:8080/api/login \
  -H 'Content-Type: application/json' \
  -d '{"username": "movindu", "password": "mypass"}' \
  -c cookies.txt
```

**Create Ticket (after login):**
```bash
curl -X POST \
  http://localhost:8080/api/tickets \
  -H 'Content-Type: application/json' \
  -d '{"title": "Learn Axum"}' \
  -b cookies.txt
```

**List Tickets:**
```bash
curl -X GET http://localhost:8080/api/tickets -b cookies.txt
```

**Delete Ticket:**
```bash
curl -X DELETE http://localhost:8080/api/tickets/0 -b cookies.txt
```

## Project Structure 📂

```
Async-Rust-With-Tokio-and-Axum/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── Tests.http
├── notes.md
└── src/
    ├── ctx.rs          # Context management (user ID)
    ├── errors.rs       # Custom error types and handling
    ├── log.rs          # Request logging utility
    ├── main.rs         # Main application entry point, routing setup
    ├── model.rs        # Data models and ModelController for ticket operations
    └── web/
        ├── mw_auth.rs  # Authentication middleware (context resolver, auth required)
        ├── route_login.rs # Login API routes
        ├── route_ticket.rs # Ticket API routes (CRUD)
        └── mod.rs      # Web module definition, AUTH_TOKEN constant
```

## API Reference 🔗

### `/api/login` (POST)
Authenticates a user and returns a session token via cookies.

### `/api/tickets` (POST, GET)
-   **POST:** Creates a new ticket.
-   **GET:** Retrieves a list of tickets.

### `/api/tickets/{id}` (DELETE)
Deletes a specific ticket by its ID.

**Note:** All `/api` routes related to tickets require authentication.

## Important Links 🔗

-   **Repository:** [Async-Rust-With-Tokio-and-Axum](https://github.com/MovinduLochana/Async-Rust-With-Tokio-and-Axum)
## Footer ®️

© 2024 Async-Rust-With-Tokio-and-Axum.

This project is maintained by [MovinduLochana](https://github.com/MovinduLochana).

Feel free to ⭐ star and 🍴 fork this repository!
