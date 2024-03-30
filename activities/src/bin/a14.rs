// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_person_info(person: &Person) {
    println!("Name: {}, Fav. Color: {}", person.name, person.color)
}

fn main() {
    let people: Vec<Person> = vec![
        Person {
            age: 5,
            name: "John".to_string(),
            color: "blue".to_string(),
        },
        Person {
            age: 20,
            name: "Jane".to_string(),
            color: "red".to_string(),
        },
        Person {
            age: 30,
            name: "Bob".to_string(),
            color: "green".to_string(),
        },
    ];

    for person in people.iter() {
        if person.age < 10 {
            print_person_info(person)
        }
    }
}
