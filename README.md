# Health Management System

## Overview

This project is a health management system. It provides a simple and efficient way to manage patient data, medical records and appointments.

It provides backend functionality serving as a bridge between the frontend and the database along other essential services such as authentication, authorization, and data validation. The backend is built using Rust, and the database is managed using PostgresDB.

## Getting Started

To get started, all you need is Docker. You can install Docker from the official website [https://docs.docker.com/engine/install/](https://docs.docker.com/engine/install/).

Once done, you can run the following command from the root directory in your terminal to start the application:

```bash
docker compose up --build
```

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
  "date": "2025-10-22 14:30:00"
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
