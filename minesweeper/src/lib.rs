pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut bombs: Vec<String> = Vec::new();
    let y = minefield.len();
    if y < 1 {
        println!("NO ROWS");
        return bombs;
    }
    let x = minefield[0].len();
    if x < 1 {
        println!("NO COLMUNS");
        bombs.push("".to_string());
        return bombs;
    }

    for i in 0..y {
        let mut line = "".to_string();
        for (j, char) in minefield[i].chars().enumerate() {
            match char {
                '*' => {
                    line.push('*');
                }
                ' ' => {
                    // println!("-------------------------- {} {}", i, j);
                    let count = check(minefield, &x, &y, &i, &j);
                    line.push(count);
                }
                _ => {
                    println!("UNKNOWN CHARACTERS");
                }
            }
        }
        bombs.push(line);
    }


    bombs
}

fn check(minefield: &[&str], x: &usize, y: &usize, i: &usize, j: &usize) -> char {
    let from_x = if *j == 0 { 0 } else { j - 1 };
    let to_x = if *j == *x-1 { *x-1 } else { j + 1 };
    let from_y = if *i == 0 { 0 } else { i - 1 };
    let to_y = if *i == *y-1 { *y-1 } else { i + 1 };

    calculate(minefield, &from_x, &to_x, &from_y, &to_y)


}

fn calculate(minefield: &[&str], from_x: &usize, to_x: &usize, from_y: &usize, to_y: &usize) -> char {
    let mut count = 0;
    for yy in *from_y..=*to_y {
        for xx in *from_x..=*to_x {
            match &minefield[yy][xx..=xx].eq("*") {
                true  => {
                    // println!("TRUE| {} {} {}", &minefield[yy][xx..=xx], xx, yy);
                    count = count + 1;
                }
                _ => {
                    // println!("ERR|| {} {} {}", &minefield[yy][xx..=xx], xx, yy);
                }
            }
        }
    }
    // println!("#### {}", count);
    match char::from_digit(count, 10) {
        Some(x) if x!='0' => return x,
        Some(_) => return ' ',
        None => '_'
    }
}