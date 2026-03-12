use surrealdb::types::{RecordId, SurrealValue, Wrapper};

#[derive(Debug, SurrealValue)]
pub struct Record {
    id: RecordId,
}

#[derive(Debug, SurrealValue)]
pub struct Class {
    pub friendly_name: String,
    pub description: String,
}

#[derive(Debug, SurrealValue)]
pub struct Student {
    pub id: RecordId,
    pub full_name: String,
    pub classes: Vec<Class>,
}

#[derive(SurrealValue)]
pub struct Schedule {
    pub weekday: Wrapper<chrono::Weekday>,
    pub start_times: Vec<Wrapper<chrono::NaiveTime>>,
}

#[derive(Debug, SurrealValue)]
pub struct AttendenceRecord {
    pub student: Student,
    pub state: Attendence,
}

#[derive(Debug, SurrealValue)]
pub enum Attendence {
    Present,
    OnCampus,
    Late,
    Excused,
    Absent,
}
