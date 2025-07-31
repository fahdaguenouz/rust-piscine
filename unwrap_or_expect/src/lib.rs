pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}
impl Security {
    pub fn verify(&self, server: Result<&str, &str>) -> String {
     Security::Unknown=>server.unwrap(),
      Security::Message => server.expect("ERROR: program stops"),
    Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
     Security::NotFound => server.unwrap_or_else(|e| panic!("Not found: {}", e)),
    Security::UnexpectedUrl => match server {
                Ok(m) => panic!("{}", m),
                Err(e) => e.to_string(),
            },
        }
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    security_level.verify(server)
}
