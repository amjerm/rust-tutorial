use std::collections::HashMap;

// Practice tasks

// 1
// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

fn first(v: Vec<i32>) -> HashMap<String, f32> {
    let mut result: HashMap<String, f32> = HashMap::new();
    let mut mean_hash: HashMap<i32, i32> = HashMap::new();

    let mut max = 0;
    let mut min = 0;

    for item in v {
        if item > max {
            max = item;
        }

        if item < min {
            min = item;
        }

        let current_value = *mean_hash.entry(item).or_insert(0);
        mean_hash.insert(item, current_value + 1);
    }

    let median = (max as f32 + min as f32) / 2 as f32;
    result.insert(String::from("median"), median);

    let mut mode = 0;
    let mut last_max = 0;

    for (key, value) in mean_hash {
        if value > last_max {
            last_max = value;
            mode = key
        }
    }

    result.insert(String::from("mode"), mode as f32);
    return result;
}

// 2
// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and "ay" is added,
// so "first" becomes "irst-fay." Words that start with a vowel have "hay" added to the end
// instead ("apple" becomes "apple-hay"). Keep in mind the details about UTF-8 encoding!

fn second(s: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_char = s.chars().next().unwrap();
    let s_split: Vec<&str> = s.split(first_char).collect();
    let rest = s_split[1];

    // if vowels.iter().any(|&i| &i != &first_char) {
    if vowels.contains(&first_char) {
        return format!("{}-hay", s);
    } else {
        return format!("{}-{}ay", String::from(rest), String::from(first_char));
    }
}

// 3
// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company.
// For example, "Add Sally to Engineering" or "Add Amir to Sales."
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

struct Company {
    employees: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            employees: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, s: &str) {
        let command_pieces: Vec<&str> = s.split(" ").collect();
        let department = command_pieces[3];
        let name = command_pieces[1];

        let employees_vec = self
            .employees
            .entry(String::from(department).to_lowercase())
            .or_insert(vec![]);
        employees_vec.push(String::from(name));
    }

    pub fn get_all_employees(&self) -> &HashMap<String, Vec<String>> {
        return &self.employees;
    }

    pub fn get_department(&self, s: &str) -> &Vec<String> {
        return &self.employees[&String::from(s).to_lowercase()];
    }
}

fn main() {
    let first_result = first(vec![2, 1, 2, 3, 5]);
    println!("The first result is {:?}", first_result);

    let second_result = second(String::from("first"));
    let second_other_result = second(String::from("apple"));
    println!(
        "The second results are {} and {}",
        second_result, second_other_result
    );

    let mut my_company = Company::new();
    println!("My company is {:?}", &my_company.get_all_employees());
    my_company.add_employee("Add Sally to Engineering");
    my_company.add_employee("Add Sally to engineering");
    my_company.add_employee("Add Amir to Sales");
    println!("My company is {:?}", &my_company.get_all_employees());
    println!(
        "The engineering department is {:?}",
        &my_company.get_department("engineering")
    );
    println!(
        "The sales department is {:?}",
        &my_company.get_department("Sales")
    );
}
