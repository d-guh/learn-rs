// This is a pretty lazy implementation, doesn't handle vowel sounding "yay" etc. also doesn't normalize case lol
fn convert(text: &str) -> String {
    text.split_whitespace().map(|word| {
        let mut chars = word.chars();
        let first_char: char = match chars.next() {
            Some(c) => c,
            None => return String::new()
        };

        if is_vowel(first_char) {
            format!("{}hay", word)
        } else {
            let mut consonants: String = String::from(first_char);
            let mut rest: String = String::new();
            let mut found_vowel: bool = false;

            for c in chars {
                if !found_vowel && !is_vowel(c) {
                    consonants.push(c);
                } else {
                    found_vowel = true;
                    rest.push(c);
                }
            }
            format!("{}{}ay", rest, consonants)
        }
    }).collect::<Vec<String>>().join(" ")
}

fn is_vowel(c: char) -> bool {
    let lower = c.to_lowercase().next().unwrap_or(c);
    matches!(lower, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn main() {
    let input = "The quick brown fox jumped over the lazy dog I guess";
    let output = convert(input);

    println!("Original: {}", input);
    println!("Pig Latin: {}", output);
}
