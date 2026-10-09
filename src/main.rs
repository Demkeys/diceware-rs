use rand::random_range;
use std::collections::HashMap;

// const TXT01: &str = include_str!("../test01.txt");

// This file contains the diceware words list. Each line contains
// an index and a word separated by a tab. The file contains
// indices 11111 to 66666.
const TXT01: &str = include_str!("../diceware-words.txt");

fn main() {
    // println!("{}", TXT01);
    // println!("Hello, world!");
    generate_passphrase();
}

fn generate_passphrase() {
    // HashMap to hold rows of index/value pairs.
    let mut txt01_hashmap: HashMap<i32, &str> = HashMap::new();

    // Split text by newline character.
    let txt01_lines = TXT01.lines();

    // For each line...
    for line in txt01_lines {
        // Split line by tab character
        let mut split = line.split('\t');

        #[allow(clippy::collapsible_if)]
        // Here we iterate over the two items, store their resulting Options into
        // a tuple and the use tuple destructuring to retrieve their values.
        if let (Some(txt_key), Some(txt_value)) = (split.next(), split.next()) {
            // Parse the key from &str to i32.
            if let Ok(key) = txt_key.parse::<i32>() {
                // Create HashMap entry using key i32 and value &str.
                txt01_hashmap.insert(key, txt_value);
            }
        }
    }

    let mut words: Vec<String> = Vec::new();

    // Five iterations to generate five words.
    for _ in 0..5 {
        let mut new_number = 0;
        // Generate five digit random number between 11111 and 66666.
        for _ in 0..5 {
            // Build five digit number by generating each digit.
            // Last operand in the formula is random between 1-6.
            // 0 * 10 + 3 = 3
            // 3 * 10 + 5 = 35
            // 35 * 10 + 3 = 353
            // 353 * 10 + 1 = 3531
            // 3531 * 10 + 2 = 35312
            new_number = (new_number * 10) + random_range(1..6);
        }

        // Index HashMap to get word and add it to words list.
        let word = txt01_hashmap.get(&new_number).unwrap();
        words.push(String::from(*word));
        // println!("{}", new_number);
        // println!("{}", word);
    }
    // Join words list items into a string separated by "-".
    let passphrase = words.join("-");

    println!("Generated passphrase: \n{}", passphrase);
    // println!("{}", TXT01.len());
    // println!("{:#?}", txt01_hashmap);
}
