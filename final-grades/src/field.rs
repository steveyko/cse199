//!
//! This defines Field enum.
//!

const LAST_NAME: &str = "Last Name";
const FIRST_NAME: &str = "First Name";
const USERNAME: &str = "Username";
const STUDENT_ID: &str = "Student ID";
const LAST_ACCESS: &str = "Last Access";
const AVAILABILITY: &str = "Availability";
const RECITATION: &str = "Recitation Section";
const WEIGHTED: &str = "Weighted Total";
const TOTAL: &str = "Total";
const TOPHAT: &str = "Top Hat";
const ATTENDANCE: &str = "TH-Attendance";
const ATT: &str = "Attendance";
const TIME: &str = "time of HW submission";

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
