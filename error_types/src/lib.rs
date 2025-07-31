use chrono::{DateTime, Local};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Local::now();
        FormError {
            form_values: (field_name.to_string(), field_value.to_string()),
            date: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: &str, password: &str) -> Form {
        Form {
            name: name.to_string(),
            password: password.to_string(),
        }
    }
    pub fn validate(&self) -> Result<(), FormError> {
        let name = &self.name;
        let password = self.password.clone();
        if self.name.trim().is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password must be at least 8 characters",
            ));
        }
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_letter && has_digit && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}
