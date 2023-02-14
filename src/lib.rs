use std::io::{self, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn check_pass() {}
}

fn get_pass() -> Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

