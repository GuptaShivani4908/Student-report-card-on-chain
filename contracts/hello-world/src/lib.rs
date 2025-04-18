#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

// Symbol for mapping report cards
const REPORT_KEY: Symbol = symbol_short!("REPORT");

#[contracttype]
#[derive(Clone)]
pub struct ReportCard {
    pub student_id: u64,
    pub name: String,
    pub subject: String,
    pub marks: u32,
    pub grade: String,
}

#[contract]
pub struct StudentReportContract;

#[contractimpl]
impl StudentReportContract {
    // Function to add a report card
    pub fn add_report_card(env: Env, student_id: u64, name: String, subject: String, marks: u32, grade: String) {
        let report = ReportCard {
            student_id,
            name,
            subject,
            marks,
            grade,
        };

        env.storage().instance().set(&(REPORT_KEY, student_id), &report);
        env.storage().instance().extend_ttl(5000, 5000);
    }

    // Function to view a student's report card by ID
    pub fn view_report_card(env: Env, student_id: u64) -> ReportCard {
        env.storage().instance().get(&(REPORT_KEY, student_id)).unwrap_or(ReportCard {
            student_id: 0,
            name: String::from_str(&env, "Not Found"),
            subject: String::from_str(&env, "N/A"),
            marks: 0,
            grade: String::from_str(&env, "N/A"),
        })
    }

    // Function to update marks and grade
    pub fn update_report_card(env: Env, student_id: u64, marks: u32, grade: String) {
        let mut report = Self::view_report_card(env.clone(), student_id);
        if report.student_id == 0 {
            panic!("Student not found");
        }

        report.marks = marks;
        report.grade = grade;

        env.storage().instance().set(&(REPORT_KEY, student_id), &report);
        env.storage().instance().extend_ttl(5000, 5000);
    }

    // Function to delete a student's report card
    pub fn delete_report_card(env: Env, student_id: u64) {
        env.storage().instance().remove(&(REPORT_KEY, student_id));
    }
}
