// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    let q1 = q.first_half.clone();
    let q2 = q.second_half.clone();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in q1 {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        for val in q2 {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}


/* threads1.rs L'erreur se trouve dans la boucle for où les résultats sont collectés dans le vecteur results. 
Pour corriger cela, vous pouvez utiliser la méthode join pour récupérer les valeurs renvoyées par chaque thread.*/

/* threads2.rs : L'erreur dans le code se trouve dans la tentative de mise à jour de la valeur partagée jobs_completed dans chaque thread 
sans prendre de précautions pour la synchronisation des accès concurrents à cette valeur.
Pour corriger cela, vous devez utiliser un mécanisme de synchronisation tel que Mutex ou Atomic
pour garantir que les threads accèdent à jobs_completed de manière sûre et sans aucune course.*/

/* threads3.rs :Dans ce code, nous avons un problème de propriété (ownership) lorsque nous passons la structure Queue à la fonction send_tx. 
//La fonction send_tx reçoit la structure Queue par valeur (q: Queue), Pour résoudre ce problème, nous devons modifier la signature de la fonction send_tx pour qu'elle accepte une référence à Queue plutôt qu'une valeur./



