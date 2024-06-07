// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Crée un Arc de Mutex pour une utilisation partagée et mutable de l'état.
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Verrouille le mutex avant de modifier la valeur partagée
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Attend que tous les travaux soient terminés
    for handle in handles {
        handle.join().unwrap();
    }

    // Imprime la valeur de `JobStatus.jobs_completed`
    let status = status.lock().unwrap();
    println!("Jobs completed: {}", status.jobs_completed);
}
