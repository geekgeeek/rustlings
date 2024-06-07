// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!");
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        Rectangle::new(10, -10);
    }
}

// tests1.rs : Pour corriger ce test, il faut passer une expression booléenne à la macro assert!
//Faire que le test compile : Passer une expression booléenne à assert!.
//Faire que le test passe : Passer une expression qui est toujours vraie.
//Faire que le test échoue : Passer une expression qui est toujours fausse

// tests3.rs : Le code actuel contient deux tests qui sont incomplets. Il manque les appels à la fonction is_even et les assertions qui vérifient les résultats attendus.
/*  corection :  Compléter le test pour vérifier si un nombre pair renvoie true.
Ajouter un test pour vérifier si un nombre impair renvoie false */
//tests4.rs : dans le test fournis, les assertions ne vérifient pas correctement ces conditions.Correction de la fonction correct_width_and_height
//Utiliser des macros d'assertion pour les tests avec des valeurs négatives
