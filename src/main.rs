use evalexpr::*;
use std::{ time::Instant };
use std::collections::HashMap;

fn check_assignment(f: &String, assignments: &Vec<bool>) -> bool {
    let mut formular = f.clone();
    for i in (0..assignments.len()).rev() {
        let assignment_string = match assignments[i] {
            true => "true",
            false => "false",
        };

        let mut internal_literal = String::from("");
        internal_literal.push_str(&i.to_string());
        formular = str::replace(&formular[..], &internal_literal[..], assignment_string);
    }
    if let Ok(eval_result) = eval(&formular[..]) {
        if let Ok(b) = eval_result.as_boolean() {
            return b;
        } else {
            panic!("could not evaluate formular")
        }
    } else {
        panic!("could not evaluate formular")
    }
}

fn is_all_true(arr: &Vec<bool>) -> bool {
    for b in arr {
        if !b {
            return false;
        }
    }
    return true;
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn get_next_assignment(assignment: Vec<bool>) -> Vec<bool> {
    if assignment.len() == 0 {
        panic!("assignment is empty");
    }
    let mut result = assignment.clone();

    for literal_index in 0..assignment.len() {
        result[literal_index] = result[literal_index] ^ true;
        if result[literal_index] {
            break;
        }
    }

    return result;
}

fn main() {
    let mut formular = String::from(
        "X1 & ( X2 | (-X3 & X4)) & ( X5 | (-X6 & X7)) & ( X5 | (-X6 & X7)) & ( X11 | (-X10 & X9)) & ( X5 | (-X6 & X8))"
    );
    remove_whitespace(&mut formular);

    let mut literals_hash: HashMap<String, String> = HashMap::new();

    // Get all different literals
    for i in 0..formular.len() {
        let c = formular.chars().nth(i).unwrap();
        if c == 'X' {
            let mut literal = String::from("X");
            for y in i + 1..formular.len() {
                let number_of_literal = formular.chars().nth(y).unwrap();
                if number_of_literal.is_numeric() {
                    literal.push(number_of_literal);
                } else {
                    break;
                }
            }
            literals_hash.insert(literal.clone(), literal.clone());
        }
    }

    // Replace Literals with an actual usabel index
    let mut literals = Vec::from_iter(literals_hash.values());
    literals.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in 0..literals.len() {
        let literal = &literals[i];

        let mut internal_literal = String::from("");
        internal_literal.push_str(&i.to_string());

        formular = str::replace(&formular[..], &literal[..], &internal_literal[..]);
    }

    // replace junctors by real bool terms from rust
    formular = str::replace(&formular[..], "&", "&&");
    formular = str::replace(&formular[..], "|", "||");
    formular = str::replace(&formular[..], "-", "!");

    // generate assignment vector
    let mut assignments: Vec<bool> = vec![false; literals.len()];
    let mut model_found;
    // let theoretical_max_loop = u32::pow(2, assignments.len() as u32);
    // println!("{}", theoretical_max_loop);

    let start = Instant::now();

    // check every assignment
    loop {
        model_found = check_assignment(&formular, &assignments);
        if model_found || is_all_true(&assignments) {
            break;
        }
        assignments = get_next_assignment(assignments);
    }

    let elapsed = start.elapsed();

    println!("Model Found {}", model_found);
    if model_found {
        println!("Model: {:?}", assignments);
    }
    println!("Took {}ms", elapsed.as_millis() as u128);
}