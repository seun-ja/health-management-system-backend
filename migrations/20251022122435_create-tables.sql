CREATE TABLE patients (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NULL,
    last_name TEXT NULL,
    age INTEGER NULL,
    class TEXT NULL,
    gender TEXT NULL,
    alergies TEXT[],
    emergency_contact TEXT NULL,
    encrypted_password TEXT NOT NULL,
    appointments TEXT[],
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE doctors (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NULL,
    last_name TEXT NULL,
    specialization TEXT NULL,
    encrypted_password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE status AS ENUM ('pending', 'ongoing', 'cancelled', 'noshow', 'completed');

CREATE TABLE appointments (
    id TEXT PRIMARY KEY,
    patient_id TEXT NOT NULL REFERENCES patients(id),
    -- doctor_id TEXT NOT NULL REFERENCES doctors(id),  -- TODO
    doctor_id TEXT NOT NULL,
    date TIMESTAMP NOT NULL,
    status status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
