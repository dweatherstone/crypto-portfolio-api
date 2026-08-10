# 📈 Crypto Portfolio & Alert API

A high-performance, asynchronous REST API built in Rust for tracking cryptocurrency portfolios, fetching real-time market prices, and managing automated price alert triggers.

Designed with clean architecture, robust error handling, and production-ready async patterns in mind.

---

## 🚀 Features

- **JWT Authentication:** Secure user signup, login, and protected route authorisation.
- **Portfolio Management:** Full CRUD operations for user holdings, buy/sell transactions, and total valuation tracking.
- **Real-Time Price Sync:** Background asynchronous worker pulling real-time exchange rates from public crypto APIs (e.g., CoinGecko / Coinbase).
- **Automated Price Alerts:** Define target price thresholds (`ABOVE` / `BELOW`) and track trigger statuses.
- **Production Observability:** Structured JSON logging, environment configuration, and compile-time SQL query validation.

---

## 🛠 Tech Stack & Crates

| Category               | Technology / Crate                                                                                              | Purpose                                   |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| **Language & Runtime** | [Rust](https://www.rust-lang.org/) / [Tokio](https://tokio.rs/)                                                 | Multi-threaded async runtime              |
| **Web Framework**      | [Axum](https://github.com/tokio-rs/axum)                                                                        | Ergonometric, modular REST web framework  |
| **Database**           | PostgreSQL + [SQLx](https://github.com/launchbadge/sqlx)                                                        | Async, compile-time checked SQL queries   |
| **Authentication**     | [jsonwebtoken](https://github.com/Keats/jsonwebtoken) + [argon2](https://github.com/RustCrypto/password-hashes) | Password hashing & JWT verification       |
| **Serialization**      | [serde](https://serde.rs/) / `serde_json`                                                                       | Data parsing and serialization            |
| **Error Handling**     | [thiserror](https://github.com/dtolnay/thiserror)                                                               | Custom, strongly-typed domain error types |
| **Observability**      | [tracing](https://github.com/tokio-rs/tracing) + `tracing-subscriber`                                           | Structured application logging            |

---

## 🏛 System Architecture Overview

```text
               +----------------------------------+
               |        Axum Web Server           |
               |  (HTTP Handlers / Middleware)    |
               +----------------+-----------------+
                                |
                                v
+------------------+   +------------------+   +------------------+
| CoinGecko / API  | < | Postgres (SQLx)  | < | Background Worker|
|  (External REST) |   |  (State / Store) |   |  (Tokio Interval)|
+------------------+   +------------------+   +------------------+
```

---

## 🛣 Development Roadmap

- **[x] Phase 1: Foundation**
  - Basic Axum setup with in-memory state (`Arc<Mutex<HashMap>>`).
  - Health check and mock portfolio endpoints.
- **[x] Phase 2: Persistence & Migrations**
  - PostgreSQL integration using `SQLx`.
  - Database schema migrations for `users`, `portfolios`, `holdings`, and `price_alerts`.
- **[ ] Phase 3: Resilient Error Handling & Logging**
  - Custom `AppError` enum converting domain errors to HTTP response codes.
  - Request logging using `tracing`.
- **[ ] Phase 4: Async Background Sync**
  - `tokio::spawn` worker fetching external prices periodically with `reqwest`.
- **[ ] Phase 5: Authentication & Security**
  - Argon2 password hashing.
  - JWT generation, validation, and route protection middleware.
- **[ ] Phase 6: Testing & Finalizing**
  - Integration tests against a live Postgres database.
  - Zero `unwrap()` calls in production code paths.

---

## 📋 API Endpoints Summary

### Auth

- `POST /api/auth/register` - Create user account
- `POST /api/auth/login` - Authenticate and receive JWT

### Portfolios & Holdings _(Protected)_

- `GET /api/portfolios` - List user portfolios
- `POST /api/portfolios` - Create a new portfolio
- `POST /api/portfolios/:id/holdings` - Add/Update a crypto holding (e.g., BTC, ETH)
- `DELETE /api/portfolios/:id/holdings/:symbol` - Remove holding

### Alerts & Prices

- `GET /api/prices` - Get latest cached cryptocurrency prices
- `POST /api/alerts` - Create a price alert _(Protected)_
- `GET /api/alerts` - List active price alerts for user _(Protected)_

---

## 💻 Getting Started Locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+)
- [Docker](https://www.docker.com/) (for local PostgreSQL instance)
- `sqlx-cli` (`cargo install sqlx-cli --no-default-features --features postgres`)

### Setup & Run

**1. Clone the repository:**

```bash
git clone https://github.com/dweatherstone/crypto-portfolio-api.git
```

**2. Configure environmnet variables:**

```bash
cp .env.example .env
```

**3. Start PostgreSQL via Docker:**

```bash
docker run --name crypto-db -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=crypto_portfolio -p 5432:5432 -d postgres:16-alpine
```

**4. Run Database Migrations:**

```bash
sqlx migrate run
```

**5. Run the Application:**

```bash
cargo run
```
