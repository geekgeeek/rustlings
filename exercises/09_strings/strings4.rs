// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
   string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    
}
/*strings1.rs : Le code ci-dessus définit une fonction current_favorite_color qui est censée 
retourner une String, mais actuellement, elle retourne une &str littérale.
Le type de retour attendu est String,
et non &str. Il faut donc convertir le littéral de chaîne "blue" en String*/

/*strings2.rs : la fonction is_a_color_word prend une référence &str comme paramètre, 
tandis que dans main, la variable word est de type String.
Le type String doit être converti en &str pour que cela fonctionne.*/

//Pour résoudre ce problème, il suffit de passer une référence de la chaîne de caractères String à la fonction is_a_color_word.
/* strings3.rs : Le code ci-dessus contient trois fonctions incomplètes trim_me,
compose_me et replace_me qui doivent manipuler des chaînes de caractères.
trim_me : Cette fonction doit enlever les espaces blancs au début et à la fin d'une chaîne.
compose_me : Cette fonction doit ajouter " world!" à la fin d'une chaîne.
replace_me : Cette fonction doit remplacer toutes les occurrences de "cars" par "balloons" dans une chaîne */

// strings4.rs : Pour ce code, la tâche consiste à déterminer si chaque valeur est un String ou un &str,
//et à appeler la fonction appropriée (string pour String, et string_slice pour &str). 
/* "blue" : C'est un littéral de chaîne &str, donc on utilise string_slice.
"red".to_string() : La méthode to_string retourne un String.
String::from("hi") : La fonction String::from retourne un String.
"rust is fun!".to_owned() : La méthode to_owned sur un littéral de chaîne retourne un String.
"nice weather".into() : La méthode into peut être utilisée pour convertir un littéral en String.
format!("Interpolation {}", "Station") : La macro format! retourne un String.*/
