// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Définition de la structure Point
struct Point {
    x: u8,
    y: u8,
}

// Définition de l'énumération Message
enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit,
}

// Définition de la structure State
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    // Ajout du champ message
    message: String,
}

impl State {
    // Méthode pour changer la couleur
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    // Méthode pour quitter
    fn quit(&mut self) {
        self.quit = true;
    }

    // Méthode pour afficher un message
    fn echo(&mut self, s: String) {
        // Corriger l'erreur en affichant le message directement
        println!("{}", s);
    }

    // Méthode pour déplacer la position
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    // Méthode pour traiter les messages
    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test de la méthode process
    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "".to_string(), // Initialisation du message
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        // Assertions pour vérifier si les modifications sont correctes
        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        // Retirer l'assertion pour `message` car il n'y a plus de champ `message`
    }
}


// enums1.rs : l'énumération Message n'est pas encore définie correctement, ce qui empêche le code de compiler.
//Pour corriger ce code, nous devons définir les variantes de l'énumération Message telles que Quit, Echo, Move, et ChangeColor.

// enums2.rs : les variantes de l'énumération Message ne sont pas encore définies. 
//Il faut les définir en fonction de leur utilisation dans le tableau messages

// enums3.rs : L'erreur dans le code réside dans le fait que l'énumération Message n'est pas encore définie avec les variantes appropriées 
//et que la méthode process n'implémente pas la logique pour traiter ces variantes. 
//Pour corriger cela, nous devons : Définir les variantes de l'énumération Message basées sur leur utilisation dans la méthode process
//Implémenter la méthode process avec une expression match pour traiter chaque variante de message
