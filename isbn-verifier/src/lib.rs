/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut check = 0;
    let mut iterator = 0;
    let chars  = isbn.chars();
    for i in chars {
        match i.is_numeric() {
            true => {
                check = check + (10 - iterator) * i.to_digit(10).unwrap();
                iterator = iterator + 1;
            },
            false if i == 'X' && iterator == 9 => {     // X value only in last position
                check = check + (10 - iterator) * 10;
                iterator = iterator + 1;
            },
            false if i == 'X' => {
                check = 12;     // invalid position of X => check=12 for enforce return false
                break;
            }
            _ => continue,
        }
    }
    if check%11 == 0 && iterator == 10{
        true
    } else {
        false
    }
}
