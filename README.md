# Networked Library Management System

A client/server library management system built in Rust, using async networking with Tokio. Multiple librarians can connect to a central server simultaneously and manage books, members, loans, reservations, and fines — all backed by persistent JSON storage.

**Course:** Rust Programming — Summer 1405
**Instructor:** Dr. Saeed Dadkhah
**Student:** Habib Azargoshasb

## Overview

The system follows a classic client/server architecture over TCP:

- **Server** — an async Tokio process that listens on a port, accepts multiple concurrent connections, and manages all library data in a shared, thread-safe state.
- **Client** — an interactive CLI application that librarians use to connect to the server and perform day-to-day operations through a simple menu.
- **Shared** — a common crate defining the data structures (`Book`, `Member`, `Loan`, etc.) and the message protocol (`Request` / `Response`) used by both client and server.

All state is persisted to disk as JSON, so data survives server restarts, and every mutating operation is timestamped in an operation log.

## Architecture

```
┌─────────┐      TCP / JSON       ┌─────────┐
│ Client  │ ───── Request ──────> │ Server  │
│  (CLI)  │ <──── Response ────── │ (Tokio) │
└─────────┘                       └────┬────┘
                                        │
                              Arc<Mutex<LibraryState>>
                                        │
                              ┌─────────┴─────────┐
                              │  library_data.json │
                              │  operation_log.txt │
                              │  report.txt        │
                              └────────────────────┘
```

Each client connection is handled in its own async task (`tokio::spawn`), so multiple librarians can work concurrently without blocking each other. Shared state is protected by a `Mutex` wrapped in an `Arc` for safe concurrent access.

## Features

### Book Management
- Add, edit, remove, and search books (by title or author)
- List all books with full details (genre, availability, ratings)

### Member Management
- Add, edit, remove, and search members

### Circulation
- Borrow and return books, with automatic due-date tracking (14-day loan period)
- Reserve books that are currently unavailable
- Optional star rating (1–5) submitted by the reader when a book is returned

### Fines
- Automatic fine calculation on overdue returns, based on days late

### Reporting & Logging
- **Generate Report** — a live statistical snapshot (total books, borrowed books, total members, active loans, total fines), written to `report.txt`
- **View Log** — a full, timestamped history of every operation performed on the system, written continuously to `operation_log.txt`

### Persistence
- The entire library state is serialized to `library_data.json` after every change and reloaded automatically on server startup — no data is lost when the server shuts down.

## Tech Stack

| Concern | Technology |
|---|---|
| Language | Rust |
| Async runtime | Tokio |
| Networking | Tokio TCP (`TcpListener` / `TcpStream`) |
| Serialization | Serde + serde_json |
| Concurrency | `Arc<Mutex<T>>` shared state |
| Persistence | JSON file I/O |

## Project Structure

```
library_v2/
├── Cargo.toml          # Workspace manifest
├── shared/             # Common data types & Request/Response protocol
│   └── src/lib.rs
├── server/              # Async TCP server
│   └── src/
│       ├── main.rs      # Connection handling & request routing
│       ├── state.rs     # LibraryState + save/load
│       └── logger.rs    # Operation logging
└── client/              # Interactive CLI client
    └── src/main.rs
```

## Getting Started

### Prerequisites
- Rust and Cargo installed ([rustup.rs](https://rustup.rs))

### Running the server

```
cargo run -p server
```

The server listens on `127.0.0.1:8080` and creates `library_data.json`, `operation_log.txt`, and `report.txt` in the working directory as needed.

### Running the client

In a separate terminal:

```
cargo run -p client
```

You'll be presented with an interactive menu:

```
=== Library Management System ===
0. Exit
1. Add Book
2. List Books
3. Borrow Book
4. Return Book
5. Remove Book
6. Edit Book
7. Search Book
8. Add Member
9. Remove Member
10. Edit Member
11. Search Member
12. Reserve Book
13. Generate Report
14. View Log
```

Multiple client instances can connect to the same server at once.

## Design Notes

- **Message protocol**: Communication uses newline-delimited JSON over TCP. Each `Request` is serialized, sent with a trailing `\n`, and the server responds the same way — the client reads exactly one line back per request.
- **Error handling**: Domain errors (book not found, already borrowed, etc.) are modeled explicitly as a `LibraryError` enum inside `Response::Error`, rather than relying on generic failures.
- **Auto-incrementing IDs**: Books and members each get a unique ID from a counter stored in `LibraryState`, avoiding ID collisions across concurrent requests.

## Planned / In Progress

- **AI Librarian Assistant** — a natural-language assistant (using `rig` with tool calling) that can answer questions like *"What books do we have about algorithms?"* by querying the live library data. Not yet implemented.

## Known Limitations

- No authentication or role-based access control — the CLI assumes anyone connecting is an authorized librarian.
- The log and report files are plain text and grow unbounded over time (no rotation).
