# 📌 FOIA Request API - Endpoints Map

## **🚀 API Overview**
| **Resource** | **Method** | **Endpoint**           | **Description**              |
| ------------ | ---------- | ---------------------- | ---------------------------- |
| **Users**    | `POST`     | `/api/users`           | Create a new user (register) |
|              | `GET`      | `/api/users`           | Get all users                |
|              | `GET`      | `/api/users/{id}`      | Get a specific user          |
|              | `PUT`      | `/api/users/{id}`      | Update user details          |
|              | `DELETE`   | `/api/users/{id}`      | Delete a user                |
| **Auth**     | `POST`     | `/api/login`           | Authenticate user            |
|              | `POST`     | `/api/forgot-password` | Request password reset       |
|              | `POST`     | `/api/reset-password`  | Reset password with token    |
| **Agencies** | `POST`     | `/api/agencies`        | Create a new agency          |
|              | `GET`      | `/api/agencies`        | Get all agencies             |
|              | `GET`      | `/api/agencies/{id}`   | Get a specific agency        |
|              | `PUT`      | `/api/agencies/{id}`   | Update an agency             |
|              | `DELETE`   | `/api/agencies/{id}`   | Delete an agency             |

---

## whats in this nightmare??
✅ Actix-Web for fast API routing    
✅ PostgreSQL database (Supabase compatible)  ✅ SQLx for async queries    
✅ Argon2 password hashing for secure authentication    
✅ Tokio async runtime for efficiency    
✅ `.env` support for environment variables

```sh
git clone https://github.com/YOUR_USERNAME/govpeep-api.gitcd govpeep-api
touch .env
```  

@alex fill out the db info on ur own im not putting that here, too riskayyyy

## dev prerequisites

```sh 
rustup updatecargo install sqlx-cli --no-default-features --features postgressqlx database createsqlx migrate run
```  

## starting backend

```sh 
cargo run
```  

should be available at http://127.0.0.1:8080

## EndPoint Map

### 🟢 User Authentication

| Method | Endpoint     | Description          |  
|--------|-------------|----------------------|  
| `POST` | `/api/signup` | Register a new user |  

### 🟢 Agency Management

| Method   | Endpoint               | Description         |  
|----------|-------------------------|---------------------|  
| `GET`    | `/api/agencies`          | Fetch all agencies |  
| `GET`    | `/api/agencies/{id}`      | Fetch a specific agency |  
| `POST`   | `/api/agencies`          | Create a new agency |  
| `PUT`    | `/api/agencies/{id}`      | Update an agency |  
| `DELETE` | `/api/agencies/{id}`      | Delete an agency |