use std::{
    fs::{self, File},
    io::{self, stdout, Result, Write},
};

#[cfg(test)]
mod tests {

    #[test]
    fn saving_pass() {
        use crate::read_saved_pass;
        use crate::save_pass;

        save_pass("testpass").unwrap();
        let saved_pass = read_saved_pass().unwrap();

        assert_eq!(saved_pass, "testpass");
    }

    #[test]
    fn check_pass() {
        use crate::is_pass_right;
        use crate::Config;

        let correct_pass = Config {
            saved_pass: "koray_123!*".to_string(),
            entered_pass: "koray_123!*".to_string(),
        };

        let incorrect_pass = Config {
            saved_pass: "koray_123!*".to_string(),
            entered_pass: "ay_123!*".to_string(),
        };

        assert_eq!(is_pass_right(correct_pass), true);
        assert_ne!(is_pass_right(incorrect_pass), true);
    }
}

pub struct Config {
    saved_pass: String,
    pub entered_pass: String,
}

impl Config {
    pub fn build() -> Result<Config> {
        if let Err(_) = read_saved_pass() {
            println!("You don't have a saved password, please enter a new one.");

            let entered_pass = enter_pass()?;
            save_pass(&entered_pass)?;
            let saved_pass = entered_pass.clone();

            return Ok(Config {
                entered_pass,
                saved_pass,
            });
        }

        let saved_pass = read_saved_pass()?;
        let entered_pass = enter_pass()?;

        save_pass(&entered_pass)?;

        Ok(Config {
            entered_pass,
            saved_pass,
        })
    }
}

fn read_saved_pass() -> Result<String> {
    fs::read_to_string("saved_pass.txt")
}
pub fn save_pass(entered_pass: &str) -> Result<&str> {
    let mut file = File::create("saved_pass.txt")?;
    file.write_all(entered_pass.as_bytes())?;
    Ok("your password has been saved !")
}

pub fn enter_pass() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let _ = stdout().flush();
    Ok(input)
}

pub fn is_pass_right(config: Config) -> bool {
    if config.entered_pass == config.saved_pass {
        return true;
    } else {
        return false;
    }
}
