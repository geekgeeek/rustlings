// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}

//primitive_types1.rs:L'erreur dans le code provient de la ligne incomplète où is_evening devrait être initialisé.
//Cette variable doit être initialisée de manière similaire à is_morning.Pour corriger le code, 
//il suffit de compléter la ligne manquante en initialisant is_evening comme une variable booléenne (true ou false).

//primitive_types2.rs :  il faut initialiser your_character avec un caractère de notre choix. exemple un emoji

//primitive_types3.rs : L'erreur vient du fait que la déclaration du tableau est manquante; 
//La ligne let a = [0; 100]; crée un tableau de 100 éléments où chaque élément est initialisé à zéro

//primitive_types4.rs: l'erreur vient du fait qu'il faut spécifier correctement les indices dans la tranche.Pour corriger ce code, 
//il faut remplacer ??? par le code qui crée une tranche contenant les éléments [2, 3, 4] du tableau a. Les indices correspondants sont 1..4

//primitive_types5.rs : l'erreur vient du fait qu'il manque le pattern de déstructuration pour assigner ses éléments
//à des variables nommées name et age. Pour corriger ce code, il faut remplacer /* your pattern here */
//par le pattern de déstructuration correct pour extraire les éléments du tuple cat et les assigner aux variables name et age.

//primitive_types6.rs : 
//Le code fourni demande d'accéder au deuxième élément du tuple numbers en utilisant la syntaxe d'indexation des tuples. 
// Les tuples en Rust sont indexés à partir de zéro, donc pour accéder au deuxième élément, nous devons utiliser l'index 1.
