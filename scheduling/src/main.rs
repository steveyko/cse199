mod class;
mod staff;

use class::*;
use staff::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        panic!("Need three arguments");
    }
    let instructor_file = args.get(1).unwrap();
    let ta_file = args.get(2).unwrap();
    let class_file = args.get(3).unwrap();

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

    // Print out the recitation assignment
    for recitation in recitations {
        println!(
            "{}, {}",
            recitation.instructor.unwrap(),
            recitation.tas.unwrap().join(",")
        );
    }

    Ok(())
}

fn assign(people: &mut Vec<Person>, recitations: &mut Vec<Recitation>) -> Result<(), String> {
    for recitation in recitations.iter_mut() {
        let index = pick_rand_index(people, &recitation)?;

        // Reduce the availability for the person
        // Assign the person to the recitation
        // Also update person info with the assigned recitation
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