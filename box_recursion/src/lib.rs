#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
         fn from(s: &str) -> Role {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker, 
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
       WorkEnvironment{
        grade:None,
       }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
       let new_worker = Box::new(Worker {
            name: name.to_string(),
            role: Role::from(role).to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(current) = self.grade.take() {
            self.grade = current.next;
            Some(current.name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        if let Some(worker) = &self.grade {
            Some((worker.name.clone(), Role::from(worker.role.as_str())))
        } else {
            None
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::CEO => "CEO",
            Role::Manager => "Manager",
            Role::Worker => "Worker",
        }
        .to_string()
    }
}