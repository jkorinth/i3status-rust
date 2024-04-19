#![allow(dead_code)]
use log::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

#[cfg(not(test))]
use std::{env, path::PathBuf};

#[cfg(not(test))]
fn read_xresources() -> std::io::Result<String> {
    use std::io::{Error, ErrorKind};
    let home = env::var("HOME")
        .map_err(|_| Error::new(ErrorKind::Other, "HOME env var was not set"))?;
    let xresources = PathBuf::from(home + "/.Xresources");
    debug!(".Xresources @ {:?}", xresources);
    std::fs::read_to_string(xresources)
}

#[cfg(test)]
use tests::read_xresources;

static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\*(?<name>[^: ]+)\s*:\s*(?<color>#[a-f0-9]{6}).*$").unwrap());

static COLORS: Lazy<Result<HashMap<String, String>, String>> =
    Lazy::new(|| match read_xresources() {
        Ok(content) => {
            debug!(".Xresources content:\n{}", content);
            return Ok(HashMap::from_iter(
                content
                    .lines()
                    .map(|line| {
                        COLOR_REGEX
                            .captures(line)
                            .map(|caps| (caps["name"].to_string(), caps["color"].to_string()))
                    })
                    .flatten(),
            ));
        }
        Err(e) => Err(format!("could not read .Xresources: {}", e)),
    });

pub fn get_color(name: &str) -> Result<Option<&String>, String> {
    Ok(COLORS.as_ref().map(|cmap| cmap.get(name))?)
}

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
        let colors = COLORS.as_ref().unwrap();
        for (name, value) in colors.iter() {
            println!("{} = {:?}", name, value);
        }
        assert!(colors.contains_key("color4"));
        assert!(colors.contains_key("background"));
        assert_eq!(2, colors.len());
    }

    #[test]
    fn test_deserializing_xcolors() {
        use super::super::color::*;
        println!("COLORS = {:?}", COLORS.as_ref().unwrap().keys());
        let _: Color = "x:background".parse().expect("can parse background");
    }
}
