fn main() {
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",];
    let gifts: [&str; 12] = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..days.len() {
        println!("On the {} day of Christmas my true love sent to me", days[i]);
        
        for j in (0..=i).rev() {
            // Probably not optimal but wanted to flow a bit better
            if j == 0 && i != 0 {
                println!("and {}", gifts[j]);
            } else if j == 0 {
                println!("{}", gifts[j]);
            } else {
                println!("{},", gifts[j]);
            }
        }
        println!();
    }
}
