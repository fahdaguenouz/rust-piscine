use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl  Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let short_hand = format!("-{}", name.chars().next().unwrap());
        let long_hand = format!("--{}", name);
        Flag {
            short_hand,
            long_hand,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand.clone(), func);
        self.flags.insert(flag.short_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
      let n1 = argv.get(0).ok_or("Missing first argument")?;
        let n2 = argv.get(1).ok_or("Missing second argument")?;
        let callback = self.flags.get(input).ok_or("Flag not found")?;

        callback(n1, n2).map_err(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
   let af: f32 = a.parse()?;
    let bf: f32 = b.parse()?;
    Ok((af / bf).to_string())

}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let af: f64 = a.parse()?;
    let bf: f64 = b.parse()?;
    Ok(format!("{:.8}", af % bf))
}