//!
//! This defines Field enum.
//!

const LAST_NAME: &'static str = "Last Name";
const FIRST_NAME: &'static str = "First Name";
const USERNAME: &'static str = "Username";
const STUDENT_ID: &'static str = "Student ID";
const LAST_ACCESS: &'static str = "Last Access";
const AVAILABILITY: &'static str = "Availability";
const RECITATION: &'static str = "Recitation Section";
const WEIGHTED: &'static str = "Weighted Total";
const TOTAL: &'static str = "Total";
const TOPHAT: &'static str = "Top Hat";
const ATTENDANCE: &'static str = "TH-Attendance";
const ATT: &'static str = "Attendance";
const TIME: &'static str = "time of HW submission";

/*
 * Only the fields of interest
 */
pub enum Field {
    LastName,
    FirstName,
    Username,
    StudentID,
    LastAccess,
    Availability,
    RecitationSection,
    WeightedTotal,
    Total,
    TopHat,
    Attendance,
    Att,
    TimeOfHWSubmission,
}

impl Field {
    pub fn from_str(s: &str) -> Option<Field> {
        match s {
            LAST_NAME => Some(Field::LastName),
            FIRST_NAME => Some(Field::FirstName),
            USERNAME => Some(Field::Username),
            STUDENT_ID => Some(Field::StudentID),
            LAST_ACCESS => Some(Field::LastAccess),
            AVAILABILITY => Some(Field::Availability),
            RECITATION => Some(Field::RecitationSection),
            WEIGHTED => Some(Field::WeightedTotal),
            TOTAL => Some(Field::Total),
            TOPHAT => Some(Field::TopHat),
            ATTENDANCE => Some(Field::Attendance),
            ATT => Some(Field::Att),
            TIME => Some(Field::TimeOfHWSubmission),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Field::LastName => LAST_NAME,
            Field::FirstName => FIRST_NAME,
            Field::Username => USERNAME,
            Field::StudentID => STUDENT_ID,
            Field::LastAccess => LAST_ACCESS,
            Field::Availability => AVAILABILITY,
            Field::RecitationSection => RECITATION,
            Field::WeightedTotal => WEIGHTED,
            Field::Total => TOTAL,
            Field::TopHat => TOPHAT,
            Field::Attendance => ATTENDANCE,
            Field::Att => ATT,
            Field::TimeOfHWSubmission => TIME,
        }
    }
}
