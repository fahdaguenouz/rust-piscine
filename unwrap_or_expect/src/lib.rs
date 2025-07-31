pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}
impl Security {
    pub fn verify(&self, server: Result<&str, &str>) -> String {
        
        match self {
            Security::Unknown => match server {
                Ok(m) => m.to_string(),
                Err(_) => panic!("UNKNOWN: server error"),
            },
            Security::Message => match server {
                Ok(m) => m.to_string(),
                Err(_) => panic!("ERROR: program stops"),
            },
            Security::Warning => match server {
                Ok(m) => m.to_string(),
                Err(_) => panic!("WARNING: check the server"),
            },
            Security::NotFound => match server {
                Ok(m) => m.to_string(),
                Err(e) => panic!("Not found: {}", e),
            },
            Security::UnexpectedUrl => match server {
                Ok(m) => m.to_string(),
                Err(e) => panic!("Unexpected URL: {}", e),
            },
        }
    }
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    security_level.verify(server)
}
