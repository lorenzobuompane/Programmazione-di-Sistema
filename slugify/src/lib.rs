pub fn slugify(s: &str) -> String {
    let mut return_value = "".to_string();

    let mut preced = 0;

    if s.len() == 0 {
        return return_value
    }
    for char in s.chars() {
        match char.is_alphanumeric() {
            true => {
                if preced == 1 { preced = 0; }
                return_value.push(conv(char));
            }
            false => {
                if preced > 0 {
                    continue;
                } else {
                    preced = 1;
                    return_value.push('-');
                }
            }
        }
    }

    if return_value.chars().last().unwrap() == '-' && s.len()!=1 {
        return_value.pop();
    }
    return_value = return_value.to_lowercase();

    return_value
}

fn conv(c: char) -> char {
    const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let mut return_value: char = c;

    for (i, char) in SUBS_I.chars().enumerate() {
        match c == char {
            true => {
                return_value = SUBS_O.chars().nth(i).unwrap()
            }
            _ => continue,
        }
    }
    return_value
}


