fn create_function1(even: bool) -> fn(i32) -> i32 {
    return if even {
        |i| { i - (i % 2) }
    } else {
        |i| { i - i % 2 + 1 }
    };
}

/*
    Box perchè il valore potrebbe avere una dimensione diversa
    dyn perchè viene utilizzato un fat-pointer
*/
fn create_function2(even: bool, base: i32) -> Box<dyn Fn(i32) -> i32> {
    return if even {
        Box::new(move |i| { i - (i % 2) + base })
    } else {
        Box::new(move |i| { i - i % 2 + 1 + base })
    };
}

fn generator(base: &str) -> impl FnMut() -> String + '_ {
    let mut x = 0;
    return move || {
        x += 1;
        format!("{}{}", base, x)
    };
}

pub fn d04m04() {
    println!("---- Lesson 04/04 ----");
    let f1 = create_function1(true);
    for i in 0..10 {
        println!("f1({}): {}", i, f1(i));
    }
    println!();
    let f2 = create_function1(false);
    for i in 0..10 {
        println!("f2({}): {}", i, f2(i));
    }
    println!("------------------------------------------------");
    let f3 = create_function2(true, 200);
    for i in 0..10 {
        println!("f3({}): {}", i, (*f3)(i));
    }
    println!();
    let f4 = create_function2(false, 200);
    for i in 0..10 {
        println!("f4({}): {}", i, (*f4)(i));
    }
    println!("------------------------------------------------");
    let mut g = generator("alfa");
    println!("{}", g());
    println!("{}", g());
    println!("{}", g());
    println!("{}", g());
    println!("{}", g());
    println!();
    let mut h = generator("beta");
    println!("{}", h());
    println!("{}", h());
    println!("{}", h());
    println!("{}", h());
    println!("{}", g());
    println!("------------------------------------------------");
}