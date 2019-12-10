//!
//! This defines Grade enum.
//!

use crate::constants::ATTENDANCE_TOTAL;

const A: &'static str = "A";
const A_MINUS: &'static str = "A-";
const B_PLUS: &'static str = "B+";
const B: &'static str = "B";
const B_MINUS: &'static str = "B-";
const C_PLUS: &'static str = "C+";
const C: &'static str = "C";
const C_MINUS: &'static str = "C-";
const D_PLUS: &'static str = "D+";
const D: &'static str = "D";
const F: &'static str = "F";

pub enum Grade {
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    F,
}

impl Grade {
    pub fn new(total: f32, attendance: usize) -> Grade {
        assert!(attendance <= ATTENDANCE_TOTAL);

        let mut grade: Grade = match total {
            x if x < 50.0 => Grade::F,
            x if x < 58.0 => Grade::D,
            x if x < 62.0 => Grade::DPlus,
            x if x < 66.0 => Grade::CMinus,
            x if x < 72.0 => Grade::C,
            x if x < 76.0 => Grade::CPlus,
            x if x < 80.0 => Grade::BMinus,
            x if x < 84.0 => Grade::B,
            x if x < 88.0 => Grade::BPlus,
            x if x < 92.0 => Grade::AMinus,
            _ => Grade::A,
        };

        let downgrade_by: usize = (ATTENDANCE_TOTAL - attendance) / 3;

        for _ in 0..downgrade_by {
            grade.downgrade();
        }

        grade
    }

    fn downgrade(&mut self) {
        *self = match self {
            Grade::A => Grade::AMinus,
            Grade::AMinus => Grade::BPlus,
            Grade::BPlus => Grade::B,
            Grade::B => Grade::BMinus,
            Grade::BMinus => Grade::CPlus,
            Grade::CPlus => Grade::C,
            Grade::C => Grade::CMinus,
            Grade::CMinus => Grade::DPlus,
            Grade::DPlus => Grade::D,
            Grade::D => Grade::F,
            Grade::F => Grade::F,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Grade::A => A,
            Grade::AMinus => A_MINUS,
            Grade::BPlus => B_PLUS,
            Grade::B => B,
            Grade::BMinus => B_MINUS,
            Grade::CPlus => C_PLUS,
            Grade::C => C,
            Grade::CMinus => C_MINUS,
            Grade::DPlus => D_PLUS,
            Grade::D => D,
            Grade::F => F,
        }
    }
}
