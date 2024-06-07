// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Complétez la signature de la fonction!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // Complétez la déclaration de sortie!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // Complétez le corps de la fonction. Vous pouvez le faire!
            match command {
                Command::Uppercase => {
                    let transformed_string = string.to_uppercase();
                    output.push(transformed_string);
                },
                Command::Trim => {
                    let transformed_string = string.trim().to_string();
                    output.push(transformed_string);
                },
                Command::Append(n) => {
                    let appended_string = string.clone() + &"bar".repeat(*n);
                    output.push(appended_string);
                },
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // Importez `transformer` pour qu'il soit en scope
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
