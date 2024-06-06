// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.
// Pour executer ce test, il faut faire cargo new generics2 --bin, modifier le fichier src/main.rs et exécuter cargo test
// I AM NOT DONE

struct Wrapper<T> { // On ajoute le parametre <T> qui représente n'importe quel type
    value: T, // value est de type T
}

impl<T> Wrapper<T> { // la methode impl <T> Wrapper<T> peut fonctionner avec tous les types
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
