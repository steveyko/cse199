use crate::class::Section;
use csv;

#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub is_instructor: bool,
    pub conflicts: Vec<Section>,
    pub availability: usize,
}

impl Person {
    pub fn new(
        name: String,
        is_instructor: bool,
        conflicts: Vec<Section>,
        availability: usize,
    ) -> Person {
        Person {
            name,
            is_instructor,
            conflicts,
            availability,
        }
    }

    fn parse_conflicts(conflicts: &str) -> Vec<Section> {
        let mut sections = vec![];

        for conflict in conflicts.split(' ') {
            match Section::parse(conflict.trim()) {
                Ok(f) => sections.push(f),
                Err(e) => panic!(e),
            };
        }

        sections
    }

    pub fn from_csv(path: &str, is_instructor: bool) -> Result<Vec<Person>, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut persons = vec![];
        for result in rdr.records() {
            let record = result?;
            let name = record.get(0).expect("Wrong CSV format for name").trim();
            let conflicts = record
                .get(1)
                .expect("Wrong CSV format for conflicts")
                .trim();
            let availability = record
                .get(2)
                .expect("Wrong CSV format for availability")
                .trim();
            if name.is_empty() || availability.is_empty() {
                panic!("Wrong CSV format");
            }

            let conflicts = if conflicts.is_empty() {
                vec![]
            } else {
                Person::parse_conflicts(&conflicts)
            };
            let availability = availability
                .parse::<usize>()
                .expect("Can't parse availability");
            persons.push(Person::new(
                name.to_string(),
                is_instructor,
                conflicts,
                availability,
            ));
        }

        Ok(persons)
    }
}
