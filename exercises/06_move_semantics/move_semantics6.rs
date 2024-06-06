// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // get_char empreinte le string sans prendre la propriété

    string_uppercase(data); // string_uppercase prend la propriété de data pour les modifications 
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) {
    let data = data.to_uppercase();

    println!("{}", data);
}
