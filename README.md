# Axum_Authenticated_Ticketing_Rest_API

Description
This project is a secure RESTful API built using the Rust Axum framework. It features a user authentication and authorization system, a ticketing system for managing support tickets, and middleware for handling authentication requirements. The project provides endpoints for ticket creation, retrieval, updating, and deletion, and includes structured error handling and context-based user tracking.

Features
  User Authentication: Implements login with token-based authentication using middleware to secure endpoints.
  Ticket Management: Supports CRUD operations for tickets, allowing authenticated users to create, retrieve, update, and delete tickets.
  Middleware: Custom middleware for handling authentication and user context resolution.
  Logging: Logs requests with UUID-based tracking and structured error reporting.
  Error Handling: Custom error types for handling various authentication and ticket-related errors.
  
Project Structure
  routes_login: Handles user login, verifying credentials, and setting authentication cookies.
  routes_ticket: Contains routes for ticket management, including creating, listing, deleting, retrieving, and updating tickets.
  mw_auth: Middleware for authentication, ensuring protected routes are accessed by authenticated users only.
  ctx: Defines the Ctx struct for user context management across requests.
  error: Custom error types with implementations for structured error responses.
  log: Functions for logging requests and errors in JSON format.

Getting Started
  Prerequisites
    Rust and Cargo
    Axum, tokio, tower-cookies
  Installation
    Clone the repository and build the project: 
      git clone https://github.com/username/axum-ticketing-system.git
      cd axum-ticketing-system
      cargo build
  Running the Server
    Run the server on 127.0.0.1:8081:
      cargo run
  Run Tests
    Execute the test suite, which sends sample HTTP requests to the API:
      cargo test -- --nocapture

API Endpoints
  Login: POST /api/login - Logs in the user and sets an authentication token.
  Create Ticket: POST /api/ticket - Creates a new ticket (requires authentication).
  List Tickets: GET /api/ticket - Lists all tickets (requires authentication).
  Get Ticket: GET /api/ticket/:id - Retrieves details of a ticket by ID (requires authentication).
  Update Ticket: PUT /api/ticket/:id - Updates an existing ticket by ID (requires authentication).
  Delete Ticket: DELETE /api/ticket/:id - Deletes a ticket by ID (requires authentication).
  
Authentication Flow
  User logs in via /api/login with valid credentials.
  An authentication token is set as a cookie.
  Middleware verifies the token for protected routes.
  Invalid or expired tokens return an error, prompting the user to re-authenticate.
  
Error Handling
  The project uses custom error types to handle and respond to errors appropriately, including:
    LoginFail: When login credentials are incorrect.
    AuthFail: For various authentication failures.
    TicketNotFound: When attempting to access a ticket that does not exist.
    
