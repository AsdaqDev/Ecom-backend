# 🔐 Rust Authentication API

> A backend authentication system built with Rust and Axum, designed with modular architecture, clear dependency boundaries, and production-oriented backend practices.

🚧 **Status: In Development**

This project is currently focused on building a complete authentication module before expanding into additional e-commerce capabilities.

The goal is not to build a large application as quickly as possible, but to understand and implement the backend fundamentals required to build maintainable production services.

---

## 🎯 Project Goals

This project is being built to explore and demonstrate:

* Rust backend development
* REST API design
* Authentication and authorization
* PostgreSQL database integration
* Password hashing with Argon2
* JWT-based authentication
* Structured error handling
* Dependency injection and application wiring
* Modular architecture
* Ports & Adapters / Hexagonal Architecture
* Testing backend business logic
* Production-oriented project structure

---

## 🛠️ Tech Stack

| Technology     | Purpose                            |
| -------------- | ---------------------------------- |
| **Rust**       | Backend language                   |
| **Axum**       | HTTP framework                     |
| **PostgreSQL** | Relational database                |
| **SQLx**       | Database access                    |
| **Argon2**     | Password hashing                   |
| **JWT**        | Authentication tokens              |
| **Tokio**      | Async runtime                      |
| **Docker**     | Development/deployment environment |

---

## 🏗️ Architecture

The project follows a **modular monolith** architecture combined with **Hexagonal Architecture (Ports & Adapters)**.

The current high-level structure is:

```text
src/
├── app/
│
├── infrastructure/
│
├── modules/
│   └── auth/
│       ├── domain/
│       ├── feature/
│       ├── port/
│       └── adapter/
│
└── shared/
```

### `app/`

Responsible for application composition and startup.

Typical responsibilities:

```text
app/
├── router
├── state
└── startup
```

The application layer is responsible for connecting the different parts of the system.

It should not contain core business logic.

---

### `infrastructure/`

Contains technology-specific infrastructure used by the application.

Examples include:

```text
infrastructure/
├── database/
├── authentication/
├── redis/
└── observability/
```

Infrastructure code deals with technologies such as PostgreSQL, SQLx, JWT libraries, and other external systems.

---

### `modules/`

Contains the business capabilities of the application.

Currently:

```text
modules/
└── auth/
```

Additional modules may be introduced as the project grows.

Each module follows the same architectural boundary:

```text
auth/
├── domain/
├── feature/
├── port/
└── adapter/
```

---

## 🔐 Auth Module

The current development focus is the authentication module.

The intended flow is:

```text
HTTP Request
     │
     ▼
   Handler
     │
     ▼
  Feature / Service
     │
     ▼
    Port
     │
     ▼
   Adapter
     │
     ▼
 PostgreSQL
```

### Domain

Contains authentication-related business concepts and rules.

Examples:

```text
domain/
├── entity/
├── value_object/
└── error/
```

The domain should remain independent from Axum, PostgreSQL, SQLx, and other infrastructure technologies.

---

### Feature

Contains application use cases.

The authentication module is being developed around features such as:

```text
feature/
├── register/
├── login/
└── logout/
```

Each feature contains the application logic required to perform that operation.

---

### Port

Defines the interfaces that the application needs from external systems.

For example:

```rust
trait UserRepository {
    // persistence operations
}
```

The feature layer depends on the abstraction rather than directly depending on PostgreSQL.

---

### Adapter

Contains implementations of those ports and HTTP-specific code.

For example:

```text
adapter/
├── postgres/
├── axum/
├── argon2/
└── jwt/
```

This is where technology-specific implementations live.

---

## 🔄 Dependency Flow

The intended dependency direction is:

```text
        Adapter
           │
       implements
           ▼
         Port
           ▲
        depends
           │
           ▼
        Feature
           │
          uses
           ▼
        Domain
```

The key principle is:

> **Business logic defines what it needs; infrastructure decides how to provide it.**

---

## 🧩 Application Wiring

Application dependencies are created and connected during startup.

For example:

```text
Configuration
      │
      ▼
  PostgreSQL Pool
      │
      ▼
 Repository
      │
      ├──────────────┐
      ▼              ▼
 PasswordHasher    JWT Service
      │              │
      └──────┬───────┘
             ▼
          AuthDeps
             │
     ┌───────┼────────┐
     ▼       ▼        ▼
 Register   Login    Logout
 Service    Service  Service
     └───────┼────────┘
             ▼
         AuthState
             │
             ▼
        Axum Router
```

This keeps dependency construction outside the business logic.

---

## 🗄️ Database

PostgreSQL is used as the primary persistence layer.

SQLx is used for database access.

Database-specific models are kept separate from domain entities where appropriate.

Conceptually:

```text
PostgreSQL
     │
     ▼
 UserModel
     │
     ▼
 Domain User
     │
     ▼
 Application Service
```

This separation prevents database representation from becoming tightly coupled to the domain model.

---

## 🔑 Authentication

The authentication system is being developed around:

* Secure password hashing with Argon2
* User registration
* Credential verification
* JWT-based authentication
* Token lifecycle management
* Authentication error handling
* Request validation

Planned authentication flow:

```text
Register
   │
   ├── Validate input
   ├── Check existing user
   ├── Hash password
   └── Persist user

Login
   │
   ├── Find user
   ├── Verify password
   └── Issue token
```

---

## ❌ Error Handling

The application uses an application-level error model rather than exposing infrastructure errors directly to API clients.

Conceptually:

```text
Repository Error
       │
       ▼
 Feature Error
       │
       ▼
  Application Error
       │
       ▼
 HTTP Response
```

Example mappings:

| Application Error |                 HTTP Status |
| ----------------- | --------------------------: |
| Validation        |           `400 Bad Request` |
| Unauthorized      |          `401 Unauthorized` |
| Forbidden         |             `403 Forbidden` |
| Not Found         |             `404 Not Found` |
| Conflict          |              `409 Conflict` |
| Internal Error    | `500 Internal Server Error` |

The API should expose safe, consistent error responses without leaking implementation details.

---

## 📋 Current Status

### Authentication

* [ ] Project configuration
* [ ] PostgreSQL connection
* [ ] Database migrations
* [ ] User model
* [ ] User repository
* [ ] Domain user entity
* [ ] Registration feature
* [ ] Password hashing
* [ ] Login feature
* [ ] JWT generation
* [ ] Logout strategy
* [ ] Authentication middleware
* [ ] Error mapping
* [ ] Request validation
* [ ] Integration tests

### Engineering

* [ ] Unit tests
* [ ] Integration tests
* [ ] API documentation
* [ ] Docker setup
* [ ] CI pipeline
* [ ] Structured logging
* [ ] Health checks

---

## 🗺️ Roadmap

### Phase 1 — Authentication

Build a complete authentication system:

```text
Register
Login
Logout
JWT
Password hashing
Validation
Error handling
Tests
```

### Phase 2 — Authorization

Introduce:

```text
Roles
Permissions
Protected routes
Authorization middleware
```

### Phase 3 — E-Commerce Modules

After the authentication foundation is stable, additional modules may be added:

```text
Users
Products
Inventory
Orders
Customers
Organizations
```

These modules will follow the same modular architecture.

---

## 🧪 Testing Strategy

The project aims to test the system at multiple levels:

```text
Unit Tests
    ↓
Feature Tests
    ↓
Repository / Database Tests
    ↓
HTTP Integration Tests
```

Important authentication cases include:

* Successful registration
* Duplicate email
* Invalid email
* Weak/invalid password
* Successful login
* Invalid credentials
* Missing authentication
* Invalid token
* Expired token

---

## 💡 Why I'm Building This

This project is being built as a practical exploration of backend engineering with Rust.

Rather than implementing a large number of CRUD endpoints, the initial focus is on understanding the boundaries between:

```text
HTTP
 ↓
Application
 ↓
Domain
 ↓
Ports
 ↓
Infrastructure
 ↓
Database
```

The project will evolve incrementally as new backend concepts are implemented and tested.

---

## 🚧 Project Status

This repository is **actively under development**.

The current milestone is the **authentication system**.

Features described in the roadmap are planned work and should not be considered implemented until they appear in the repository's code and tests.

---

## 👨‍💻 Author

**Asdaq**

Building backend systems with Rust while exploring:

* Backend architecture
* Distributed-system fundamentals
* Database design
* Authentication
* API development
* Software engineering practices

---

## 📄 License

This project is currently intended as a learning and portfolio project.
