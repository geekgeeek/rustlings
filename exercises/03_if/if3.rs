// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2                     // nous allons utiliser un seul type : i32
    } else if animal == "snake" {
        3
    } else {
        0                     // nous allons utiliser un seul type : i32
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}



//if1.rs : L'erreur est que la fonction bigger n'est pas implémentée. 
//Il faut compléter cette fonction  (utuliser la structure if-else) pour qu'elle retourne le plus grand des deux nombres fournis

//if2.rs : les erreurs sur ce code sont : la fonction foo_if_fizz doit retourner une chaîne de caractères (&str).
//Cependant, dans la branche else, elle retourne 1, qui est un entier (i32).
//La fonction foo_if_fizz doit retourner "bar" pour l'entrée "fuzz" et "baz". pour toutes les autres entrées. Actuellement, ces cas ne sont pas gérés
//Pour résoudre ces problèmes, nous avons : Corriger les types de retour pour qu'ils soient tous &str.
// et Ajouter des branches else if pour gérer les différents cas.

//if3.rs : l'erreur vient du fait que le variable "identifier"est censé avoir un type unique,
//mais à plusieur type (i32, f64, &str), pour corriger cela on peut utiliser un seul type 
//pour identifier. Le type le plus approprié serait un entier (i32).


