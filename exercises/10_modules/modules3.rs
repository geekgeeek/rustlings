// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/* modules1.rs : L'erreur ici est due à la visibilité des fonctions dans le module sausage_factory. 
Par défaut, les éléments (fonctions, structs, etc.) dans un module sont privés. 
Pour permettre à la fonction make_sausage d'être appelée depuis l'extérieur du module,
nous devons la rendre publique en utilisant le mot-clé pub.*/

/* modules2.rs :Les erreurs proviennent de la mauvaise utilisation de la syntaxe use et as.
Voici les corrections nécessaires : Importation correcte avec use et as :
Utiliser le mot-clé as pour donner un nouveau nom aux constantes importées.
Définir les imports comme publics : Assurez-vous que les imports sont publics
pour qu'ils soient accessibles dans main. */

/* modules3.rs: 
Le code essaie d'utiliser SystemTime et UNIX_EPOCH du module std::time. Pour que le code compile correctement, 
il faut importer SystemTime et UNIX_EPOCH à partir de std::time en utilisant l'instruction use. */
