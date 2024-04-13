#![allow(dead_code)]
use log::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, env, fs::File, io::Read, path::PathBuf};

static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\*(?<name>[^:]+)\s*:\s*(?<color>#[a-f0-9]{6}).*$").unwrap());

pub static COLORS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let home = env::var("HOME").expect("HOME env var was not set?!");
    let xresources = PathBuf::from(home + "/.Xresources");
    debug!(".Xresources @ {:?}", xresources);
    if xresources.exists() {
        let mut content: String = String::new();
        File::open(xresources)
            .expect("")
            .read_to_string(&mut content)
            .unwrap();
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

    #[test]
    fn test_reading_colors() {
        env_logger::init();
        for (name, value) in COLORS.iter() {
            println!("{} = {:?}", name, value);
        }
    }
}
