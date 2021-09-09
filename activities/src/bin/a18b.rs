// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
enum EmployeeCategory {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Technician,
}

struct Employee {
    category: EmployeeCategory,
    active: bool,
}
fn allowed_category(emp: &Employee) -> Result<(), String> {
    match emp.category {
        EmployeeCategory::Maintenance => Ok(()),
        EmployeeCategory::Marketing => Ok(()),
        EmployeeCategory::Manager => Ok(()),
        _ => Err(String::from("wrong category")),
    }
}

fn active_employee(emp: &Employee) -> Result<(), String> {
    match emp.active {
        true => Ok(()),
        false => Err(String::from("terminated")),
    }
}
fn can_enter(employee: &Employee) -> Result<(), String> {
    allowed_category(&employee)?;
    active_employee(&employee)?;
    Ok(())
}

fn main() {
    let kate = Employee {
        category: EmployeeCategory::Kitchen,
        active: false,
    };

    match can_enter(&kate) {
        Ok(()) => println!("allowed"),
        Err(msg) => println!("not allowed: {}", msg),
    }
}
