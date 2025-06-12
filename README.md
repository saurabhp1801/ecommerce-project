# Rust-based microservices eCommerce system with:

PostgreSQL for database

Docker for containerization

A Gateway service for routing

# Microservices:

order-service

inventory-service

notification-service

product-catalog-service

payment-service

# Each Microservice Uses: 

Axum Framework

tokio for async runtime

sqlx for Postgres (we’ll use sqlx for simplicity)

Environment config with dotenv or config

Separate routes, handlers, models, services, and DB modules