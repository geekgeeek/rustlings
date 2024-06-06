// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: &(impl SomeTrait + OtherTrait)) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

/* traits1.rs : L'implémentation du trait AppendBar pour le type String est manquante.
Pour corriger cela, il faut implémenter la fonction append_bar de manière à ce qu'elle ajoute "Bar" à la fin d'une chaîne de caractères String. */

/* traits2.rs : L'implémentation du trait AppendBar pour un vecteur de chaînes de caractères (Vec<String>) est manquante. 
Pour le resoudre ilfaut l'implementer. */

/* traits3.rs : L'erreur dans ce code est que le trait Licensed n'est pas entièrement implémenté pour les structures SomeSoftware et OtherSoftware.
Nous allons ajouter une méthode par défaut pour licensing_info dans le trait Licensed. 
Cette méthode par défaut renverra la même information pour les deux structures*/

/* traits4.rs : L'erreur dans ce code réside dans la fonction compare_license_types, où les types des paramètres software et software_two ne sont pas spécifiés.
Pour corriger cela, nous devons indiquer que ces paramètres doivent implémenter le trait Licensed*/

/* traits5.rs : L'erreur dans ce code est que la fonction some_func attend un paramètre qui implémente à la fois SomeTrait et OtherTrait,
Pour que le code compile, nous devons spécifier que le paramètre item doit implémenter les deux traits.*/


