use chrono::Local;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError<'a> {
    pub form_values: (&'a str, &'a str),
    pub date: String,
    pub err: &'a str,
}

impl<'a> FormError<'a> {
    pub fn new(field_name: &'a str, field_value: &'a str, err: &'a str) -> Self {
        let now = Local::now();
        FormError {
            form_values: (field_name, field_value),
            date: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

impl<'a> Form<'a> {
    pub fn new(name: &'a str, password: &'a str) -> Form<'a> {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError<'a>> {
        if self.name.trim().is_empty() {
            return Err(FormError::new(
                "name",
                self.name,
                "Username is empty",
            ));
        }
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password,
                "Password must be at least 8 characters",
            ));
        }
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_letter && has_digit && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password,
                "Password must contain ASCII letters, numbers and symbols",
            ));
        }

        Ok(())
    }
}
