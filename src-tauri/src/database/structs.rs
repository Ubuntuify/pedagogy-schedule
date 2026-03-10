use surrealdb::types::{RecordId, SurrealValue, Wrapper};

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

#[derive(SurrealValue)]
pub struct Schedule {
    weekday: Wrapper<chrono::Weekday>,
    start_times: Vec<Wrapper<chrono::NaiveTime>>,
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
