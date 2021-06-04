use csv;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub enum Section {
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    C1,
    C2,
    C3,
    C4,
    D1,
    D2,
    D3,
    D4,
    E1,
    E2,
    E3,
    E4,
    F1,
    F2,
    F3,
    F4,
}

impl Section {
    pub fn parse(section: &str) -> Result<Section, String> {
        match section {
            "A1" => Ok(Section::A1),
            "A2" => Ok(Section::A2),
            "A3" => Ok(Section::A3),
            "A4" => Ok(Section::A4),
            "B1" => Ok(Section::B1),
            "B2" => Ok(Section::B2),
            "B3" => Ok(Section::B3),
            "B4" => Ok(Section::B4),
            "C1" => Ok(Section::C1),
            "C2" => Ok(Section::C2),
            "C3" => Ok(Section::C3),
            "C4" => Ok(Section::C4),
            "D1" => Ok(Section::D1),
            "D2" => Ok(Section::D2),
            "D3" => Ok(Section::D3),
            "D4" => Ok(Section::D4),
            "E1" => Ok(Section::E1),
            "E2" => Ok(Section::E2),
            "E3" => Ok(Section::E3),
            "E4" => Ok(Section::E4),
            "F1" => Ok(Section::F1),
            "F2" => Ok(Section::F2),
            "F3" => Ok(Section::F3),
            "F4" => Ok(Section::F4),
            _ => Err(format!("{}: No such section", section)),
        }
    }
}

#[derive(Clone)]
pub struct Recitation {
    pub section: Section,
    pub conflicts: Option<Vec<Section>>,
    time: String,
    pub instructor: Option<String>,
    pub tas: Option<Vec<String>>,
}

impl Recitation {
    pub fn new(
        section: Section,
        conflicts: Option<Vec<Section>>,
        time: String,
        instructor: Option<String>,
        tas: Option<Vec<String>>,
    ) -> Recitation {
        Recitation {
            section,
            conflicts,
            time,
            instructor,
            tas,
        }
    }

    pub fn from_csv(path: &str) -> Result<Vec<Recitation>, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut recitations = vec![];

        use std::collections::HashMap;
        let mut time_map: HashMap<String, Vec<Section>> = HashMap::new();
        for result in rdr.records() {
            let record = result?;
            let section = record
                .get(0)
                .expect("Wrong CSV format for recitation")
                .trim();
            let section = match Section::parse(section) {
                Ok(f) => f,
                Err(e) => panic!("{}", e),
            };
            let time = record.get(1).expect("Wrong CSV format for time").trim();
            let time = time.to_string();
            recitations.push(Recitation::new(section, None, time.clone(), None, None));
            if let Some(conflicts) = time_map.get_mut(&time) {
                conflicts.push(section);
            } else {
                time_map.insert(time, vec![section]);
            }
        }

        // Get other recitations that overlap
        for recitation in recitations.iter_mut() {
            if let Some(conflicts) = time_map.get(&recitation.time) {
                recitation.conflicts = Some(conflicts.clone());
            } else {
                recitation.conflicts = None;
            }
        }

        Ok(recitations)
    }
}
