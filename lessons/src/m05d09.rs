use std::thread::Scope;
use std::thread;
use std::time::Duration;

fn scope_fun() {
    let v = std::sync::Mutex::new(vec![1, 2, 3]);
    std::thread::scope(|s: &Scope| {
        s.spawn(|| {
            for i in 10..28 {
                v.lock().unwrap().push(i);
                thread::sleep(Duration::from_nanos(1));
            }
        });
        s.spawn(|| {
            for i in 30..40 {
                v.lock().unwrap().push(i);
                thread::sleep(Duration::from_nanos(1));
            }
        });
    });
    let v = v.lock().unwrap();
    for i in &*v {
        println!("{}", i);
    }
}

pub fn m05d09() {
    println!("---- Lesson 08/05 ----");
    scope_fun();
}