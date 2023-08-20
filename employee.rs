use std::io;

#[derive(Debug)]
struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee data (or type 'quit' to exit):");

        let mut input = String::new();
        println!("Employee Name:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let employee_name = input.trim().to_string();

        if employee_name.to_lowercase() == "quit" {
            break;
        }

        println!("Employee ID:");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let employee_id: u32 = input.trim().parse().expect("Invalid input");

        println!("Email:");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let email = input.trim().to_string();

        println!("Age:");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let age: u32 = input.trim().parse().expect("Invalid input");

        println!("Phone Number:");
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let phone_number = input.trim().to_string();

        let employee = Employee {
            employee_id,
            employee_name,
            email,
            age,
            phone_number,
        };
        employees.push(employee);
    }

    println!("Employee Data:");
    for employee in &employees {
        println!("ID: {}", employee.employee_id);
        println!("Name: {}", employee.employee_name);
        println!("Email: {}", employee.email);
        println!("Age: {}", employee.age);
        println!("Phone Number: {}", employee.phone_number);
    }

    match get_employee_by_id(&employees, 2) {
        Some(employee) => {
            println!("Employee found by ID:");
            println!("ID: {}", employee.employee_id);
            println!("Name: {}", employee.employee_name);
            println!("Email: {}", employee.email);
            println!("Age: {}", employee.age);
            println!("Phone Number: {}", employee.phone_number);
        }
        None => println!("Employee not found by ID"),
    }

    let employees_with_same_age = get_employees_by_age(&employees, 30);
    println!("Employees with the same age:");
    for employee in employees_with_same_age {
        println!("ID: {}", employee.employee_id);
        println!("Name: {}", employee.employee_name);
        println!("Email: {}", employee.email);
        println!("Age: {}", employee.age);
        println!("Phone Number: {}", employee.phone_number);
    }
}

fn get_employee_by_id(employees: &Vec<Employee>, target_id: u32) -> Option<&Employee> {
    employees
        .iter()
        .find(|&employee| employee.employee_id == target_id)
}

fn get_employees_by_age(employees: &Vec<Employee>, target_age: u32) -> Vec<&Employee> {
    employees
        .iter()
        .filter(|&employee| employee.age == target_age)
        .collect()
}
