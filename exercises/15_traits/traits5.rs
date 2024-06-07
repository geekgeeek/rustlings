// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: &impl Licensed, software_two: &impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(&some_software, &other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(&other_software, &some_software));
    }
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


