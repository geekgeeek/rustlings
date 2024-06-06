// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
// Rewrite Wrapper using generics to support any type.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T>{
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

/* generics1.rs : Le code est un programme simple qui essaie de créer une liste de courses (shopping_list) et d'y ajouter un élément ("milk"). Cependant, le type du vecteur (Vec) n'est pas spécifié, 
    ce qui empêche la compilation. Dans ce cas, nous ajoutons une chaîne de caractères (&str) à la liste, donc le type du vecteur doit être &str. */
    /* generics2.rs : Le code actuel définit une structure Wrapper qui ne peut contenir qu'un entier non signé de 32 bits (u32). 
La tâche consiste à réécrire cette structure en utilisant des génériques afin qu'elle puisse contenir n'importe quel type. */
