use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::Thread;
use std::time::Duration;

fn cvar_print_new_data() {
    let pair = Arc::new(
        (
            Mutex::new(Vec::new()),
            Condvar::new()
        ));
    let pair2 = pair.clone();

    let t = thread::spawn(move || {
        let (m, cv) = &*pair2;
        for i in 0..10 {
            // thread::sleep(Duration::from_secs(3));           // senza questo cosa succede? non ne genera più solo 1 valore per volta ma è più veloce del risveglio dell'altro
            thread::sleep(Duration::from_nanos(1));
            let mut v = m.lock().unwrap();
            v.push(i);
            cv.notify_all();            // di solito se non sappiamo bene, FUNZIONA SEMPRE (anche se meno efficiente)
        }
    });

    let (m, cv) = &*pair;
    let mut round = 0;
    while round != 10 {
        let mut v = m.lock().unwrap();
        while round == v.len() {
            v = cv.wait(v).unwrap();
        }

        println!("While sleeping {} values have been produced", v.len() - round);
        for i in round..v.len() {
            println!("{}", v[i]);
        }
        round = v.len();
    }
    t.join().unwrap();
}

fn chanell_print_new_data() {
    let (tx, rx) = channel();
    let t1 = thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
        thread::sleep(Duration::from_secs(3));
    });

    while let Ok(i) = rx.recv() {
        println!("Ricevuto {i}");
    }
    t1.join().unwrap();
}

pub fn d23m05() {
    println!("---- Lesson 23/05 ----");
    // cvar_print_new_data();
    chanell_print_new_data();
}