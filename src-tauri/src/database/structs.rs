use surrealdb_types::{RecordId, SurrealValue, Value};

#[derive(Debug, SurrealValue)]
pub struct Class {
    friendly_name: String,
    description: String,
}

#[derive(Debug, SurrealValue)]
pub struct Student {
    id: RecordId,
    full_name: String,
    classes: Vec<Class>,
}

#[derive(Debug, SurrealValue)]
pub struct Schedule {
    num_of_time_slots: u32,
}

#[derive(Debug, SurrealValue)]
pub struct AttendenceRecord {
    student: Student,
    state: Attendence,
}

#[derive(Debug, SurrealValue)]
pub enum Attendence {
    Present,
    OnCampus,
    Late,
    Excused,
    Absent,
}
