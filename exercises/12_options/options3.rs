// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
/* options1.rs : Utiliser une condition pour vérifier les différentes plages d'heures et retourner les valeurs appropriées en utilisant Some ou None.
On doit déstructurer l'Option pour obtenir la valeur contenue en utilisant un match ou unwrap*/

/* options2.rs : Les erreurs se trouvent dans les if let et while let statements incomplets.
Test simple_option :Vérifie si une variable de type Option contient une valeur et, si c'est le cas, compare cette valeur avec une cible.
Test layered_option : Manipule un vecteur de Options pour vérifier les valeurs tout en retirant des éléments du vecteur */


/* options3.rs : L'erreur vient du fait que, après le match, 
la variable y n'est plus utilisable car elle a été déplacée lors de la correspondance avec Some(p).
Pour corriger cela sans supprimer la ligne y; on peut faire correspondre une référence à Some dans le match. */
