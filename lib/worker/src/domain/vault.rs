use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Vault {
    ns: String,
    rel: String,
}

impl Vault {
    pub fn from(s: String) -> Result<Vault, String> {
        let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)[.]([a-zA-Z_][a-zA-Z0-9_]*$)").unwrap();
        if re.is_match(&s) {
            let parts: Vec<&str> = s.split('.').collect();
            Ok(Vault {
                ns: parts[0].to_string(),
                rel: parts[1].to_string(),
            })
        } else {
            Err(format!("{} is not a valid vault.", s))
        }
    }

    pub fn namespace(&self) -> String {
        self.ns.clone()
    }

    pub fn relation(&self) -> String {
        self.rel.clone()
    }
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.ns, self.rel)
    }
}
