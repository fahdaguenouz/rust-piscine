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
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Box::new(Worker {
            name: name.to_string(),
            role: role.to_string(),
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
    match &self.grade {
        Some(worker_box) => {
            let worker = &**worker_box;

            if worker.role == "CEO" {
                Some((worker.name.clone(), Role::CEO))
            } else if worker.role == "Manager" {
                Some((worker.name.clone(), Role::Manager))
            } else {
                Some((worker.name.clone(), Role::Worker))
            }
        }
        None => None,
    }
}
}
