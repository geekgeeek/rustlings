// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}


//Corection des erreurs

//functions1.rs : L'erreur dans le code  est que la fonction "call_me()" n'est pas définie. Pour corriger cette erreur,
//Il faut  définir la fonction call_me() avant de l'appeler dans la fonction main().

//functions2.rs: l'erreur se trouve dans la definition de la fonction "call_me".
//Il faut spécifier le type de num comme i32 (ou tout autre type entier approprié) :

//functions3.rs : L'erreur dans votre code est due au fait que la fonction "call_me" 
//attend un argument de type u32, mais elle est appelée sans argument dans la fonction main. 
//Pour le corriger il faut fournir un argument lors de l'appel.

//functions4.rs : l'erreur se trouve au niveau de la fonction sale_price qui  est incorrecte 
//car il manque le type de retour après l'opérateur "->" qui est i32

//functions5.rs : l'erreur se trouve sur la ligne "num * num;",  dans la fonction square ne retourne pas la valeur
//car le point-virgule indique que c'est une instruction, pas une expression de retour. Pour retourner la valeur de l'expression, 
//il suffit de retirer le point-virgule à la fin de l'expression "num * num"
