// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // 
    string("red".to_string()); //convertis en string
    string(String::from("hi")); //créer un string
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station")); //produit un string
    string_slice(&String::from("abc")[0..1]); // on coupe un string, c'est donc un &str
    string_slice("  hello there ".trim()); //retourne un &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); 
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // .to_lowercase retourne un string
}
