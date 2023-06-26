fn with_iter() {
    println!("iter()");
    let v = vec!["a", "b", "c"];

    for i in v.iter() {
        println!("{}", i);
    }

    println!("Il valore finale di v è {:?}", v);
    println!("------------------------------------");
}

fn with_into_iter() {
    println!("into_iter()");
    let v = vec!["a", "b", "c"];

    for i in v.into_iter() {
        println!("{}", i);
    }

    // println!("Il valore finale di v è {:?}", v); ---> NON POSSO STAMPARE PERCHè VIENE CONSUMATO
    println!("------------------------------------");
}

#[derive(Debug)]
struct T(i32);

impl Drop for T {
    fn drop(&mut self) {
        println!("Dropping T{}", self.0);
    }
}

fn with_iter_struct() {
    println!("struct()");
    let mut v = vec![T(1), T(2), T(3)];

    for i in v.iter_mut() {
        println!("{:?}", i);
        i.0 += 3;
    }

    // ALLA FINE DEL PROGRAMMA VIENE DROPPATA OGNI ISTANZA DEL VEC
    println!("Il valore finale di v è {:?}", v);
    println!("------------------------------------");
}

fn with_into_iter_struct() {
    println!("struct()");
    let v = vec![T(1), T(2), T(3)];

    for i in v.into_iter() {
        println!("{:?}", i);
    }

    // println!("Il valore finale di v è {:?}", v); ---> NON POSSO STAMPARE PERCHè VIENE CONSUMATO
    // AD OGNI PASSO, DOPO AVER STAMPATO, L'ISTANZA VIENE DROPPATA
    println!("Fine");
    println!("------------------------------------");
}

fn with_into_iter_struct_break() {
    println!("struct()");
    let v = vec![T(1), T(2), T(3)];

    for i in v.into_iter() {
        println!("{:?}", i);
        if i.0 == 1 { break; }          // MI FERMO QUANDO IL CONTENUTO è =1 (nel nostro caso al primo passo), COSA SUCCEDE?
        // NON LASCIA UNA COSA MEZZA DROPPATA, QUINDI IN CASO DI break IL VEC VIENE DROPPATO COMPLETAMENTE
    }

    // println!("Il valore finale di v è {:?}", v); ---> NON POSSO STAMPARE PERCHè VIENE CONSUMATO
    // AD OGNI PASSO, DOPO AVER STAMPATO, L'ISTANZA VIENE DROPPATA
    println!("Fine");
    println!("------------------------------------");
}

pub fn m04d25() {
    println!("---- Lesson 25/04 ----");
    with_iter();
    with_into_iter();
    with_iter_struct();
    with_into_iter_struct();
    with_into_iter_struct_break();
}