# Energo Fit - Backend

Energo Fit Backend is a Rust-based API service that handles all backend operations for the Energo Fit workout tracking platform. This backend serves as the data manager and authentication handler, providing a robust API to the frontend. It works in conjunction with a PostgreSQL database and offers endpoints for managing workouts, exercises, user statistics, and authentication.

## Technologies

The backend is developed using the following technologies and libraries:

- **Rust**: Main programming language.
- **Actix**: Web framework used for handling HTTP requests and responses.
- **Diesel**: ORM (Object-Relational Mapping) for interacting with the PostgreSQL database.
- **Tokio**: Asynchronous runtime used for handling async tasks.
- **jsonwebtoken**: For JWT-based user authentication.
- **Rust-argon2**: For hashing passwords securely.
- **serde** and **serde_json**: For serialization and deserialization.
- **dotenv**: For environment variable management.

## Project Structure

The backend is structured to provide clear separation between different aspects of the application, such as request handling, data models, and business logic.

## Prerequisites

### Database Setup
- The backend works with a **PostgreSQL** database.
- All migrations required for setting up the database are available in the `migrations` folder. You can run these migrations using Diesel CLI:
  ```bash
  diesel migration run

### Docker Container
- The backend is available as a Docker container: `octaneal/fitness-app-backend`.
- This container can be used to quickly deploy the backend along with the frontend.

### Integration with Frontend
- The backend is designed to work in tandem with the [Energo Fit Frontend](https://github.com/OctaneAL/fitness-app-frontend). Make sure both the frontend and backend are running for the application to work correctly.

## Main Features
- User Authentication:
  - User registration and login with JWT tokens.
- Workouts Management:
  - Endpoints for creating, editing, deleting and retrieving workout sessions.
- Exercise Catalog:
  - Endpoints for fetching, searching, and filtering exercises from the extensive exercise database.
- Statistics:
  - Provides user-specific workout statistics, including progress tracking, favorite exercises, and more.

## Endpoints Overview
The backend API is organized into different endpoint categories:
- `/statistics/...`: Provides access to various user statistics.
- `/workout/...`: Handles all workout-related actions (adding, editing, deleting).
- `/exercises/...`: Allows access to the exercise catalog with filters and search functionality.

## Getting Started

The project using this BackEnd is already deployed and available for use at [Energo Fit - Live](http://ec2-51-20-193-148.eu-north-1.compute.amazonaws.com/).

However, if you'd like to run the project locally, you can use Docker:

- The Docker container is available under the name `octaneal/fitness-app-backend`.

## Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/energo-fit-frontend.git
   cd energo-fit-frontend

2. Create your own `.env` file in the root directory with the following variables:
   ```bash
   SECRET_KEY=<your_secret_key>  # Key for password hashing and JWT tokens
   DATABASE_URL=<your_database_url>  # For connecting to the database

3. Set up your environment variables by creating a `.env` file in the root directory.

4. Run the backend:
   ```bash
   cargo run

## Final Notes
Energo Fit Backend is designed to be efficient and scalable, providing a comprehensive set of features for managing physical workouts. The Rust and Actix stack ensures performance and reliability, making it a great fit for fitness tracking applications.
