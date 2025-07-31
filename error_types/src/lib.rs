use chrono::Utc;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
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
        if self.name.trim().is_empty() {
            return Err(FormError {
                form_values: ("name", self.name.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty",
            });
        }
        
        if self.password.len() < 8 {
            return Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long",
            });
        }
        
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_letter && has_digit && has_symbol) {
            return Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            });
        }

        Ok(())
    }
}