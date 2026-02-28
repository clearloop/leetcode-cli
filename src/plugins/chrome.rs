use crate::{Error, Result};
use std::{collections::HashMap, fmt::Display};

/// Spawn cookies to cookie format
#[derive(Debug)]
pub struct Ident {
    pub csrf: String,
    session: String,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LEETCODE_SESSION={};csrftoken={};",
            self.session, self.csrf
        )
    }
}

/// Get cookies from chrome storage
pub fn cookies() -> Result<Ident> {
    let ccfg = crate::config::Config::locate()?.cookies;
    if !ccfg.csrf.is_empty() && !ccfg.session.is_empty() {
        return Ok(Ident {
            csrf: ccfg.csrf,
            session: ccfg.session,
        });
    }

    let mut m: HashMap<String, String> = HashMap::new();

    let domains = vec![format!("{}", ccfg.site)];
    let cookies = rookie::chrome(Some(domains)).unwrap();
    for c in cookies {
        if (c.name == "csrftoken") || (c.name == "LEETCODE_SESSION") {
            m.insert(c.name, c.value);
        }
    }

    Ok(Ident {
        csrf: m.get("csrftoken").ok_or(Error::ChromeNotLogin)?.to_string(),
        session: m
            .get("LEETCODE_SESSION")
            .ok_or(Error::ChromeNotLogin)?
            .to_string(),
    })
}

