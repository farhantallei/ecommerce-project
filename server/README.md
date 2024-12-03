# Rust Diesel Project

This project uses Diesel as the ORM for interacting with the PostgreSQL database.

## Prerequisites

- Rust and Cargo installed
- PostgreSQL installed and running
- Diesel CLI installed

## Installation

1. **Set up the environment variables:**

   Copy the `.env.example` file to `.env` and update the `DATABASE_URL` with your database credentials.

    ```sh
    cp .env.example .env
    ```

2. **Install Diesel CLI:**

    ```sh
    cargo install diesel_cli --no-default-features --features postgres
    ```

3. **Run database migrations:**

    ```sh
    diesel migration run
    ```

## Running the Project

1. **Build and run the project:**

    ```sh
    cargo run
    ```

2. **Running tests:**

    ```sh
    cargo test
    ```

[//]: # (## Project Structure)

[//]: # ()
[//]: # (- `src/schema.rs`: Contains the database schema definitions.)

[//]: # (- `.env`: Contains environment variables including the database URL.)

## Useful Commands

- **Creating a new migration:**

    ```sh
    diesel migration generate migration_name
    ```

- **Running migrations:**

    ```sh
    diesel migration run
    ```

- **Reverting the last migration:**

    ```sh
    diesel migration revert
    ```

## Documentation

To view the API documentation, start the server and navigate to the main page, e.g.:
```
http://localhost:3000/
```

## License

This project is licensed under the MIT License.
