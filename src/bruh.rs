use rand::Rng;

fn is_polish_letter(c: char) -> bool {
    let c = c.to_lowercase().next().unwrap();
    return c == 'ą'
        || c == 'ć'
        || c == 'ę'
        || c == 'ł'
        || c == 'ń'
        || c == 'ó'
        || c == 'ś'
        || c == 'ź'
        || c == 'ż';
}

fn is_letter(c: char) -> bool {
    let c = c.to_lowercase().next().unwrap();
    return (c >= 'a' && c <= 'z') || is_polish_letter(c);
}

fn replace_with_underlines(input: String, percentage: u8) -> String {
    if percentage > 100 {
        panic!("Percentage should be between 0 and 100.");
    }

    let mut rng = rand::thread_rng();
    let mut chars: Vec<char> = input.chars().collect();
    let letter_count = chars.iter().filter(|&&c| is_letter(c)).count();
    let mut chars_to_replace = (letter_count * percentage as usize + 50) / 100;

    for i in 0..chars.len() {
        if is_letter(chars[i]) && rng.gen_range(0..letter_count) < chars_to_replace {
            chars[i] = '_';
            chars_to_replace -= 1;
        }
    }

    return chars.into_iter().collect();
}

fn main() {
    let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
    let output = replace_with_underlines(input, 25);
    println!("{}", output);
}
