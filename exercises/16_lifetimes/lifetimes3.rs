// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

/* lifetimes1.rs : Pour corriger le code, nous devons ajouter des annotations de durée de vie explicites à la fonction longest.
Ajouter une durée de vie explicite 'a qui est appliquée à toutes les références dans la signature de la fonction.
Cette durée de vie indique que la référence retournée par la fonction longest vivra aussi longtemps que la plus courte des références passées en paramètres.*/


/* lifetimes2.rs : Le problème se situe dans la fonction main. La variable string2 est créée dans un bloc interne, ce qui signifie qu'elle sera détruite à la fin de ce bloc. Cependant, result tente d'utiliser une référence à string2 en dehors de ce bloc,
ce qui conduit à une référence invalide. Une solution consiste à déplacer la déclaration de string2 en dehors du bloc interne. */

/* lifetimes3.rs : L'erreur provient du fait que les champs de la struct Book contiennent des références (&str),
mais les durées de vie de ces références ne sont pas spécifiées. 
Pour corriger ce problème, nous devons ajouter des annotations de durée de vie à la struct Book
afin que Rust puisse vérifier que les références restent valides aussi longtemps que la struct Book existe.*/
