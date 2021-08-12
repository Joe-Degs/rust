use std::error::Error;

mod employee;
use employee::{Employee, EmployeeRecords, Human, Labourer};

/// Borrow a value that implements the Employee trait and print
/// the id of the borrowed value.
/// first time i got a borrow thing right on rust.
fn print_id(x: &impl Employee) {
    println!("Id of employee is: {}", x.id())
}

/// take a value on the heap that implements the trait Employee
fn print_id_box(x: Box<&dyn Employee>) {
    println!("Id of employee on the heap: {}", x.id())
}

/// main returns a Result enum on completion. The good part of the
/// result contains nothing on succesful run but on error, the
/// error it returns an object on the heap that implements an Error
/// trait.
fn main() -> Result<(), Box<dyn Error>> {
    let emp = Human::new(String::from("Joseph Attah"), 67);

    // lend out emp to print_id so it uses it and gives it
    // back after use.
    print_id(&emp);

    // borrow emp, put it on the heap and do something with it.
    // return it to parent after wards.
    print_id_box(Box::new(&emp));

    // wow rust can do stuff like this and its cool.
    // This borrows this value and just prints it out because
    // i tweaked the i64 type to implement Employee
    print_id(&200);
    print_id_box(Box::new(&200));

    // i64 is extended so we can call the id method on i64 values.
    let x: i64 = 200;
    println!("i64 extended with Employee: {}", x.id());

    // Labourer is composed of the human struct with an empty name.
    let lab = Labourer::new(192929);

    print_id(&lab);
    print_id_box(Box::new(&lab));

    // at this stage emp is returned to its parent scope so we can
    // use it.
    println!("employee name: {}\nid: {}", emp.name(), emp.id());

    let emp_recs =
        EmployeeRecords::new_with_vec(String::from("Joe Company"), Labourer::get_labourers(200));

    println!("employees: {}", emp_recs.name());
    println!("sample labourers: {:#?}", Labourer::get_labourers(3));
    Ok(())
}
