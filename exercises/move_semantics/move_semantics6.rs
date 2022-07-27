// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// I AM NOT DONE

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&mut data);

    string_uppercase(data); // after that data, is invalid


// Should not take ownership
fn get_char(data: &mut String) { // & is terminal
    data.chars().last().unwrap();
}

// Should take ownership
fn string_uppercase(data: String) {
    let mut data = data.to_uppercase(); // shadowing

    println!("{}", data);
}
