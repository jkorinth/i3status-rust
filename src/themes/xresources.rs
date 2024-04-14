#![allow(dead_code)]
use log::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, env, path::PathBuf};

#[cfg(not(test))]
fn read_xresources() -> std::io::Result<String> {
    let home = env::var("HOME").expect("HOME env var was not set?!");
    let xresources = PathBuf::from(home + "/.Xresources");
    debug!(".Xresources @ {:?}", xresources);
    return std::fs::read_to_string(xresources);
}

#[cfg(test)]
use tests::read_xresources;

static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\*(?<name>[^:]+)\s*:\s*(?<color>#[a-f0-9]{6}).*$").unwrap());

pub static COLORS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let home = env::var("HOME").expect("HOME env var was not set?!");
    let xresources = PathBuf::from(home + "/.Xresources");
    debug!(".Xresources @ {:?}", xresources);
    if let Ok(content) = read_xresources() {
        debug!(".Xresources content:\n{}", content);
        return HashMap::from_iter(
            content
                .split("\n")
                .map(|line| {
                    COLOR_REGEX
                        .captures(line)
                        .map(|caps| (caps["name"].to_string(), caps["color"].to_string()))
                })
                .flatten(),
        );
    }
    warn!(".Xresources not found");
    HashMap::new()
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    pub(crate) fn read_xresources() -> Result<String> {
        static XRESOURCES: &str = "\
        ! this is a comment\n\
        \n\
        *color4 : #feedda\n\
    \n\
        *background: #ee33aa\n\
        ";
        Ok(XRESOURCES.to_string())
    }

    #[test]
    fn test_reading_colors() {
        for (name, value) in COLORS.iter() {
            println!("{} = {:?}", name, value);
        }
    }

    #[test]
    fn test_deserializing_xcolors() {
        use super::super::color::*;
        println!("COLORS = {:?}", COLORS.keys());
        let _: Color = "x:background".parse().expect("can parse background");
    }
}
