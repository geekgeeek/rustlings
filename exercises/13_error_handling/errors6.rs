// errors6.rs
//
// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here, we
// define a custom error type to make it possible for callers to decide what to
// do next when our function returns an error.
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}

/* errors1.rs : L'erreur dans le code est que la fonction generate_nametag_text retourne actuellement Option<String>,
mais dans le test explains_why_generating_nametag_text_fails, il est attendu que la fonction retourne un Result<String, String>,
où le résultat est une erreur avec un message explicatif. Pour corriger cela, il faut modifier la signature de la fonction generate_nametag_text pour qu'elle retourne un Result<String, String> 
et utiliser Err pour retourner une erreur avec un message explicatif lorsque le nom est une chaîne vide. */

/* errors2.rs : L'erreur dans ce code est que la fonction total_cost retourne toujours Ok quelle que soit la validité de la chaîne fournie. */
//Pour corriger cela, nous devons utiliser le résultat obtenu après la tentative de conversion de la chaîne en entier (qty). Si la conversion réussit//

/* errors3.rs :L'erreur réside dans l'utilisation de l'opérateur ? dans la fonction main(). L'opérateur ? est utilisé pour propager les erreurs,
//Pour résoudre cela, nous devons utiliser une gestion manuelle des erreurs dans la fonction main(). Cela signifie que nous devons extraire manuellement le résultat de total_cost et traiter les erreurs à l'aide d'un bloc match ou d'une méthode comme unwrap()

//errors4.rs : 
//L'erreur dans cette méthode se situe dans la conversion de value en u64. Même si value est négatif, la conversion en u64 produit toujours une valeur positiv.
//Pour corriger cela, nous devons ajouter des vérifications pour détecter les valeurs négatives et les valeurs nulles

//errors5.rs : Le principal problème à résoudre dans ce code est de choisir le trait approprié pour les erreurs possibles que peut rencontrer la fonction main(). Dans ce cas, les erreurs possibles sont de type CreationError et ParseIntError.
/* errors6.rs:L'erreur dans ce code est que la fonction de conversion d'erreur pour ParseIntError n'est pas implémentée. 
