use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@gmail.com"),
        phone: String::from("1234"),
        id: 1,
    });
    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@gmail.com"),
        phone: String::from("5678"),
        id: 2,
    });
    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@gmail.com"),
        phone: String::from("9876"),
        id: 3,
    });
    students.push(Student {
        name: String::from("David"),
        email: String::from("david@gmail.com"),
        phone: String::from("4321"),
        id: 4,
    });
    students.push(Student {
        name: String::from("Eve"),
        email: String::from("eve@gmail.com"),
        phone: String::from("8765"),
        id: 5,
    });

    loop {
        println!("Enter the index of the student to view (0-4) or 'quit' to exit:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_lowercase() == "quit" {
            break;
        }

        match input.trim().parse::<usize>() {
            Ok(index) => {
                if index < students.len() {
                    let student = &students[index];
                    println!("Student {} Details:", index);
                    println!("Name: {}", student.name);
                    println!("Email: {}", student.email);
                    println!("Phone: {}", student.phone);
                    println!("ID: {}", student.id);
                } else {
                    println!("Index out of bounds. Please enter a valid index (0-4).");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid index (0-4) or 'quit' to exit.");
            }
        }
    }
}
