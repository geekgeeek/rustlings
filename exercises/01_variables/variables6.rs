// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


fn main() {
    const NUMBER: i32 = 3;
    println!("Number {}", NUMBER);
}


//variable1.rs : L'erreur se trouve sur la ligne où x est assignée une valeur (x = 5;). La variable doit etre déclaré avant utilisation.
//variable2.rs : L'erreur se trouve sur la ligne if x == 10, La variable x est déclarée mais n'a pas reçu de valeur initiale , ce qui cause une erreur de compilation.
//variable3.rs : L'erreur dans ce code se situe dans l'utilisation de la variable x avant qu'elle ne soit initialisée, car cela pourrait entraîner un comportement indéfini
//variable4.rs : L'erreur dans ce code se situe sur la ligne x = 5; En Rust, les variables sont immuables par défaut, pour changer la variable après initialisation il faut la déclarer comme mutable.
//variable5.rs :L'erreur dans ce code se situe sur la ligne number = 3 ; Une fois qu'une variable est déclarée avec un type, ce dernier devient inchangeable.
//Dans ce code, number est initialisé en tant que chaîne de caractères (&str), mais on essaie ensuite de lui assigner une valeur de type entier (i32). 
//variable6.rs : L'erreur se trouve dans la ligne const NUMBER = 3;. Rust nécessite que le type de la constante soit spécifié lors de sa déclaration.
