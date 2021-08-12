#[derive(Debug, Clone)]
pub struct Human {
    name: String,
    id: i64,
}

impl Human {
    pub fn new(name: String, id: i64) -> Self {
        Human { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Labourer {
    human: Human,
}

/// EmployeeRecords keeps a record of a list of employees a company has.
/// It does this by keeping a vector values on the heap that have implemented
/// the Employee trait.
pub struct EmployeeRecords {
    /// name of the employer requiring the employees.
    employer_name: String,

    /// Index to keep track in iteration.
    idx: usize,

    /// recs is a vector of values that
    /// implement the Employee trait.
    recs: Vec<Labourer>,
}

impl EmployeeRecords {
    /// Get a new employee records struct with an empty
    /// employee list.
    pub fn new(name: String) -> Self {
        Self {
            employer_name: name,
            idx: 0,
            recs: vec![],
        }
    }

    /// Return EmployeeRecords with a predefined list of employees.
    pub fn new_with_vec(name: String, employees: Vec<Labourer>) -> Self {
        Self {
            employer_name: name,
            idx: 0,
            recs: employees,
        }
    }

    /// return a reference of a string type
    pub fn name(&self) -> &str {
        &self.employer_name
    }

    /// Add new employee to the employee list.
    pub fn add(&mut self, employee: Labourer) {
        self.recs.push(employee)
    }
}

/// Implement the Iterator type for Employee records so we can
/// loop over the elements in the records.
impl Iterator for EmployeeRecords {
    type Item = Box<Labourer>;

    /// next is the only required method on Iterator trait.
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.recs.len() {
            self.idx += 1;
            Some(Box::new(self.recs[self.idx - 1].clone()))
        } else {
            None
        }
    }
}

/// Labourer implements employee.
impl Labourer {
    pub fn new(id: i64) -> Self {
        Labourer {
            human: Human {
                name: String::from(""),
                id: id,
            },
        }
    }

    /// get a bunch of labourers.
    pub fn get_employees(n: usize) -> Vec<Box<dyn Employee>> {
        let mut employees: Vec<Box<dyn Employee>> = vec![];
        for ii in 0..n {
            employees.push(Box::new(Labourer::new(ii as i64)))
        }
        employees
    }

    /// Get vector of labourers.
    pub fn get_labourers(n: usize) -> Vec<Labourer> {
        let mut labs = Vec::<Labourer>::new();
        for ii in 0..n {
            labs.push(Labourer::new(ii as i64))
        }
        labs
    }

    /// get the id of the labourer
    pub fn get_id(&self) -> i64 {
        self.human.get_id()
    }
}

/// Trait for Employee's is that they have id's.
pub trait Employee {
    // fn name(&self) -> &str;
    fn id(&self) -> i64;
}

/// Implement the Debug for dyn Employee types.
/// Copied from the internet and it works.
use core::fmt::Debug;
impl Debug for dyn Employee {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Id: {}", self.id())
    }
}

impl Employee for Human {
    fn id(&self) -> i64 {
        self.get_id()
    }
}

impl Employee for Labourer {
    fn id(&self) -> i64 {
        self.get_id()
    }
}

impl Employee for i64 {
    fn id(&self) -> Self {
        *self
    }
}
