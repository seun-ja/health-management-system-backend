# Health Management System

## Overview

This project is a health management system. It provides a simple and efficient way to manage patient data, medical records and appointments.

It provides backend functionality serving as a bridge between the frontend and the database along other essential services such as authentication, authorization, and data validation. The backend is built using Rust, and the database is managed using PostgresDB.

## Getting Started

### Clone the repository

Ensure you have `git` installed on your machine by running `git --version`. If you don't have `git` installed, you can download it from the official website [https://git-scm.com/downloads](https://git-scm.com/downloads).

Clone the repository using this `git` command:

```bash
git clone https://github.com/seun-ja/health-management-system-backend
```

### Create Docker Image

To get started, all you need is Docker. You can install Docker from the official website [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/).

Once done, you can run the following command from the root directory in your terminal to start the application:

```bash
docker compose up --build
```

This exposes our API to http://localhost:8080.

## Usage

First we check it's working properly by running the following command in your terminal:

```bash
curl http://localhost:8080/ping
```

You should see the following output:

```bash
pong
```

other routes available are:

### Signup

Endpoint to create a new user

```bash
[POST] /signup
```

request body:

```json
{
  "email": "firstemail@gmail.com",
  "password": "lngerignrgowngnew",
  "first_name": "John",
  "last_name": "Doe",
  "age": 23,
  "class": "SS3",
  "gender": "Male",
  "alergies": [],
  "emergency_contact": "Alice"
}
```

response: UserID

```json
"019a1065-08f0-7c70-a487-7e3fc7d1d480"
```

### Login

Endpoint to authenticate a user

```bash
[POST] /login
```

request body:

```json
{
  "email": "firstemail@gmail.com",
  "password": "lngerignrgowngnew"
}
```

response: JWT Token

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIwMTlhMTA2NS0wOGYwLTdjNzAtYTQ4Ny03ZTNmYzdkMWQ0ODAiLCJlbWFpbCI6ImZpcnN0ZW1haWxAZ21haWwuY29tIiwiZXhwIjoxNzYxMjk4MDc2fQ.7_zUc-eWoHScbHlBsa2wVj9xWuKLEPBUWCK8y6-XQQI"
}
```

### Appointments

Endpoint to manage patients

```bash
[POST] /appointment
```

request body:

```json
{
  "patient_id": "019a1065-08f0-7c70-a487-7e3fc7d1d480",
  "date": "2025-10-22 14:30:00",
  "preferred_doctor_id": "059a1535-08f0-7c70-a445-7e3fc7d1d480" // <-- Optional
}
```

response

```json
{
  "appointment_id": "019a1065-c7d8-7740-9a67-aea4c565edff",
  "doctor_first_name": "Bob",
  "date": "2025-10-22 14:30:00"
}
```

## Questions & Contribution

This project is open source and welcomes contributions from the community. If you have any questions or suggestions, please feel free to open an issue or submit a pull request or send an email to hello@oluseun.dev.
