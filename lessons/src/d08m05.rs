use std::thread;
use std::time::Duration;

fn run(msg: &str, limit: std::sync::Arc<i32>) {
    for i in 0..*limit {
        println!("{}{}", msg, i + 1);
        thread::sleep(Duration::from_nanos(1));
    }
}

fn thread_execution() {
    let a1 = std::sync::Arc::<i32>::new(10);
    let b1 = a1.clone();
    let t1 = thread::spawn(|| { run("aaaa", a1); });
    let t2 = thread::spawn(|| { run(" bbbb", b1); });
    t1.join().unwrap();
    t2.join().unwrap();
}

fn verify_casual_schedule_order(n: usize) {
    println!(" || VERIFY CASUAL SCHEDULE ORDER ||");
    for i in 0..n {
        println!("#{}", i + 1);
        thread_execution();
    }
    println!("----------------------------------");
}


pub fn d08m05() {
    println!("---- Lesson 08/05 ----");
    verify_casual_schedule_order(3);
}