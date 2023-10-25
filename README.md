# Rocket App with SurrealDB: TODO App

![Run Dev](./.github/todo.png)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://www.rust-lang.org/tools/install)
- [SurrealDB](https://surrealdb.com/)


## Setup

1. **Clone the Repository**: Clone the repository to your local machine.

2. **Start SurrealDB:

   ```bash
   surreal start memory -A --auth --user root --pass root --bind 0.0.0.0:8001
   ```

   ![SurrealDB](./.github/surreal.png)
3. Start the Rocket web server:


   ```bash
   cargo run
   ```
   ![Rocket](./.github/rocket.png)
