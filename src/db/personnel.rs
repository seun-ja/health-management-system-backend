use uuid::Uuid;

use crate::schema::model::MedicalStaff;

pub fn personnels() -> Vec<MedicalStaff> {
    vec![
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "john.doe@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "jane.smith@example.com".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "alice.wonderland@example.com".to_string(),
            first_name: "Alice".to_string(),
            last_name: "Wonderland".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "bob.smith@example.com".to_string(),
            first_name: "Bob".to_string(),
            last_name: "Smith".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "charlie.brown@example.com".to_string(),
            first_name: "Charlie".to_string(),
            last_name: "Brown".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "david.jones@example.com".to_string(),
            first_name: "David".to_string(),
            last_name: "Jones".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
        MedicalStaff {
            id: Uuid::now_v7(),
            email: "emily.davis@example.com".to_string(),
            first_name: "Emily".to_string(),
            last_name: "Davis".to_string(),
            specialization: "Cardiology".to_string(),
            encrypted_password: "hashed_password".to_string(),
        },
    ]
}
