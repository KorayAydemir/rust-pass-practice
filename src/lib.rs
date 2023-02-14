use std::{
    fs::File,
    io::{self, stdout, Result, Write},
};

#[cfg(test)]
mod tests {
    #[test]
    fn check_pass_test() {
        let saved_pass = "koray_123!*";
        let entered_pass = "koray_123!*";

        assert_eq!(check_pass(&saved_pass, &entered_pass), true);
    }
}

pub fn save_pass(entered_pass: &str) -> Result<&str> {
    let mut file = File::create("saved_pass.txt")?;
    file.write_all(entered_pass.as_bytes())?;
    Ok("your password has been saved !")
}

pub fn get_pass() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let _ = stdout().flush();
    Ok(input)
}
