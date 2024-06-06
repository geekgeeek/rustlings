// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let mut data = "Rust is great!".to_string(); // Changement en mutable

    let last_char = get_char(&data); // Passer une référence immuable
    println!("Last character: {}", last_char);

    string_uppercase(&mut data); // Passer une référence mutable

    println!("Uppercase string: {}", data);
}

// Ne prend pas possession, utilise une référence immuable
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Prend une référence mutable pour modifier la chaîne
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();
    println!("Uppercase inside function: {}", data);
}

/* move_semantics1.rs : L'erreur dans le code est que la fonction fill_vec prend le vecteur vec en tant que paramètre mutabl, ce qui signifie qu'elle en prend possession (ownership) et peut donc le modifier. Cependant, après avoir pris possession du vecteur, elle essaie d'en créer 
une nouvelle variable vec en réassignant la valeur de vec à elle-même, ce qui revient à essayer de le prendre en possession une seconde fois. 
La correction est de supprimer la ligne let vec = vec; *//

/* move_semantics2.rs : L'erreur dans ce code est plus subtile. Elle réside dans la fonction main où la variable vec0 est utilisée après avoir été passée à fill_vec.
 Pour résoudre ce problème, il faut utiliser la méthode clone() pour créer une copie de vec0 avant de la passer à fill_vec. Cela garantit que la variable originale reste inchangée. */

/* move_semantics3.rs : L'erreur dans ce code se situe dans la fonction fill_vec. Actuellement, la fonction essaie de pousser 88 dans le vecteur vec, 
Maintenant, la fonction fill_vec prend un référence mutable (&mut Vec<i32>) plutôt qu'un vecteur appartenant (Vec<i32>). Cela permet à la fonction de modifier le vecteur sans le posséder, résolvant ainsi l'erreur de compilation. */

/* move_semantics4.rs : L'erreur dans ce code est que la fonction fill_vec est définie pour ne pas prendre d'argument,
mais elle utilise néanmoins une variable vec à l'intérieur de la fonction. Cette variable vec n'est pas définie dans la portée de la fonction. 
Pour résoudre cela, nous devons créer le vecteur à l'intérieur de la fonction fill_vec. Voici le code corrigé 
 */

/* move_semantics5.rs : Ce code comporte une erreur subtile. Il semble qu'il tente d'ajouter 100 à la variable x à travers y et 1000 à travers z, ce qui donnerait 1200 pour la valeur finale de x. Cependant, l'erreur réside dans le fait que Rust ne permet pas d'avoir deux références mutables (&mut) à la même variable en même temps, ce qui est le cas ici.

Pour résoudre ce problème, nous devons nous assurer que z n'a pas de référence mutable à x en même temps que y. */


/* move_semantics5.rs : La fonction get_char prend possession de la chaîne de caractères (String) data, mais le but est simplement de lire le dernier caractère sans prendre possession de toute la chaîne. Pour résoudre cela, nous devons passer data par référence plutôt que par valeur. De plus, la fonction devrait retourner un caractère (char) plutôt qu'une chaîne (String)*/
