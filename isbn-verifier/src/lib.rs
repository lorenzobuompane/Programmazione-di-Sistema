/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut check = 0;
    let mut iterator = 0;
    let chars = isbn.chars();
    for i in chars {
        match i {
            '0'..='9' => {
                check = check + (10 - iterator) * i.to_digit(10).unwrap();
                iterator = iterator + 1;
            }
            'X' if iterator == 9 => {
                check = check + 10; 
                iterator = iterator + 1;
            }
            '-' => {
                continue;
            }
            _ => {
                check = 12;
                break;
            }
        }
    }
    if check % 11 == 0 && iterator == 10 {
        true
    } else {
        false
    }
}
