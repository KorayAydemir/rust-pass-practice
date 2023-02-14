use rust_pass_practice::get_pass;
use rust_pass_practice::save_pass;

fn main() {
    let entered_pass = get_pass().expect("An error occured while reading the password");

    save_pass(&entered_pass).unwrap_or_else(|err| {
        eprintln!("Problem saving the password: {err}");
        std::process::exit(1)
    });
}
