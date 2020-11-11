extern crate clap;

use clap::{App, Arg};

mod class;
mod staff;

use class::*;
use staff::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("scheduler")
        .arg(
            Arg::with_name("instructors")
                .help("CSV file that contains instructor schedule conflicts")
                .required(true),
        )
        .arg(
            Arg::with_name("TAs")
                .help("CSV file that contains TA schedule conflicts")
                .required(true),
        )
        .arg(
            Arg::with_name("classes")
                .help("CSV file that contains a class schedule")
                .required(true),
        )
        .get_matches();
    let instructor_file = matches.value_of("instructors").unwrap();
    let ta_file = matches.value_of("TAs").unwrap();
    let class_file = matches.value_of("classes").unwrap();

    // Assign instructors
    let mut recitations;
    loop {
        let mut instructors = Person::from_csv(instructor_file, true)?;
        recitations = Recitation::from_csv(class_file)?;
        match assign(&mut instructors, &mut recitations) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }

    // Assign TAs
    loop {
        let mut tas = Person::from_csv(ta_file, false)?;
        match assign(&mut tas, &mut recitations) {
            Ok(_) => match assign(&mut tas, &mut recitations) {
                Ok(_) => break,
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }

    // Print out the recitation assignment for instructors
    println!("Instructors:\n");
    for recitation in &recitations {
        println!("{}", recitation.instructor.as_ref().unwrap());
    }

    // Print out the recitation assignment for TAs
    println!("\nTAs:\n");
    for recitation in &recitations {
        println!("{}", recitation.tas.as_ref().unwrap().join(", "));
    }

    Ok(())
}

fn assign(people: &mut Vec<Person>, recitations: &mut Vec<Recitation>) -> Result<(), String> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    recitations.shuffle(&mut rng);
    for recitation in recitations.iter_mut() {
        let index = pick_rand_index(people, &recitation)?;

        // Reduce the availability for the person
        // Assign the person to the recitation
        // Also update person info with overlapping recitations
        if let Some(person) = people.get_mut(index) {
            person.availability -= 1;
            if person.is_instructor {
                recitation.instructor = Some(person.name.clone());
            } else {
                if let Some(tas) = &mut recitation.tas {
                    tas.push(person.name.clone());
                } else {
                    recitation.tas = Some(vec![person.name.clone()]);
                }
            }
            if let Some(conflicts) = &recitation.conflicts {
                person.conflicts.extend(conflicts.iter().copied());
            }
        } else {
            panic!("person index out of bounds");
        }
    }
    recitations.sort_by(|a, b| a.section.cmp(&b.section));

    Ok(())
}

fn pick_rand_index(people: &Vec<Person>, recitation: &Recitation) -> Result<usize, String> {
    // Get people without conflicts (their indices)
    let mut available_people: Vec<usize> = vec![];
    for (index, person) in people.iter().enumerate() {
        if !person.conflicts.contains(&recitation.section) && person.availability != 0 {
            available_people.push(index);
        }
    }

    if available_people.len() == 0 {
        return Err("No available people".to_string());
    }

    // Pick a random person
    use rand::prelude::*;
    let pick: usize = random::<usize>() % available_people.len();
    if let Some(index) = available_people.get(pick) {
        Ok(*index)
    } else {
        panic!("available_people index out of bounds");
    }
}
