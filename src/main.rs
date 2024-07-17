use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

fn get_phonetic_dictionary() -> HashMap<String, Vec<String>>{
    let exe_dir = env::
        current_exe()
        .expect("Unable to get exe directory.");

    let exe_dir = exe_dir
        .ancestors()
        .nth(1)
        .expect("Unable to get exe directory.");    

    // Now in the crate dir, go down to the dict values
    let mut cmudict = PathBuf::from(exe_dir);
    cmudict.push("cmudict-master");
    cmudict.push("cmudict.dict");

    // Open and buffer the file
    let file = File::open(cmudict).expect("Unable to open cmudict.");
    let reader = BufReader::new(file);

    // Create a new HashMap for later conversion
    let mut phonetic_converter: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let mut key = "";

        for (num, token) in line
            .unwrap_or("".to_string())
            .split_whitespace()
            .enumerate() 
        {
            // The first token is the key
            if num == 0 {
                key = token;
            // Otherwise, as long as it isn't commented out we keep it
            } else if token != "#" {
                phonetic_converter
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(token.to_string());
            // If we find the comment token we break
            } else {
                break;
            }
        }
    }

    phonetic_converter
}


fn get_phonetic_vowels() -> Vec<String> {
    let exe_dir = env::
        current_exe()
        .expect("Unable to get exe directory.");

    let exe_dir = exe_dir
        .ancestors()
        .nth(1)
        .expect("Unable to get exe directory.");

    // Now in the crate dir, go down to the phonemes file
    let mut cmu_vowels = PathBuf::from(exe_dir);
    cmu_vowels.push("cmudict-master");
    cmu_vowels.push("cmudict.phones");

    let file = File::open(cmu_vowels)
        .expect("Unable to open {cmu_vowels}");
    let buffer = BufReader::new(file);

    let mut return_vec: Vec<String> = Vec::new();

    for line in buffer.lines() {
        let tokens: Vec<String> = line
            .unwrap_or(String::from(""))
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if tokens.contains(&"vowel".to_string()) {
            return_vec.push(tokens.get(0).unwrap().clone());
        }
    }

    return_vec
}


fn capitalise_word(word: &mut String) {
    if let Some(first_char) = word.chars().next() {
        *word = first_char.to_uppercase().collect::<String>() + &word[1..];
    }
}


fn pig_latinize(word: &String, vowel_initial: bool) -> String {
    // Make a new string to give to the caller
    let mut return_string = String::new();

    let mut new_word_capitalised = false;
    if let Some(first_char) = word.chars().next() {
        new_word_capitalised = first_char.is_uppercase();
    }

    // Discard borrowed word for a new all lowercase one
    let word = word.to_lowercase();

    //if it starts with a vowel, just add -hay to the end
    if vowel_initial {
        return_string.push_str(&word);
        return_string.push_str("-hay");
        if new_word_capitalised{
            capitalise_word(&mut return_string);
        }
        return_string
    // Otherwise, move the first letter to the back and add ay
    } else {
        let mut original_chars = word.chars();
        
        let mut initial = String::new();
        if word.len() >= 2 {
            let beginning = &word[0..2];
            let digraph_exceptions = vec![
                "ch",
                "gh",
                "kn",
                "ph",
                "qu",
                "sh",
                "th",
                "wh",
                "wr"
            ];
            let digraph: bool = digraph_exceptions.contains(&beginning);

            initial = if digraph {
                original_chars.next();
                original_chars.next();
                String::from(beginning)
            } else {
                original_chars.next();
                String::from(&beginning[..1])
            };
        }

        let rest: String = original_chars.collect();

        return_string.push_str(&rest);
        return_string.push_str("-");
        return_string.push_str(&initial);
        return_string.push_str("ay");
        if new_word_capitalised{
            capitalise_word(&mut return_string);
        }
        return_string
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args = VecDeque::from(args);
    args.pop_front();

    if args.len() < 1 {
        println!("Expected a file name; terminating.");
        return;
    }

    // Load in the various bits of data used to read
    // determine how words should be transformed.
    let phonetic_spellings = get_phonetic_dictionary();
    let vowel_phonetic: Vec<String> = get_phonetic_vowels();
    let vowel_chararcter: Vec<String> = vec!["a", "e", "i", "o", "u"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    for arg in args {
        let file_to_read = match File::open(arg.clone()) {
            Ok(file) => file,
            Err(_) => {
                println!("Skipping {}, can't be read", arg);
                continue;
            }
        };

        let mut output_vec: Vec<String> = Vec::new();
        let buffer = BufReader::new(file_to_read);

        for line in buffer.lines() {
            let mut new_line = String::new();

            // Iterate through each token in a line
            for word in line
                .unwrap_or("".to_string())
                .split_whitespace()
            {

                // Determine token the word starts with a vowel or not
                let vowel_initial: bool = match phonetic_spellings.get(&word.to_lowercase()) {
                    Some(val) => {
                        let first_phoneme = val
                            .get(0)
                            .expect("Phoneme list empty {word.clone()}")
                            .chars()
                            .filter(|&c| !c.is_digit(10))
                            .collect();
                        vowel_phonetic.contains(&first_phoneme)
                    },
                    None => {
                        let first_character = word
                            .chars()
                            .next()
                            .expect("No characters in word {word.clone()}");
                        vowel_chararcter.contains(&first_character.to_string().to_lowercase())
                    }
                };

                let mut raw_word: String = word
                    .to_string()
                    .chars()
                    .collect();

                if let Some(last_char) = raw_word.chars().last() {
                    if last_char.is_ascii_punctuation() {
                        raw_word.pop();
                    }
                }

                let punctuation = word
                    .to_string()
                    .chars()
                    .last()
                    .filter(|&c| c.is_ascii_punctuation());

                let new_word = pig_latinize(&raw_word, vowel_initial);

                new_line.push_str(&new_word);
                if punctuation.is_some() {
                    new_line.push(punctuation.unwrap())
                }
                new_line.push(' ');
            }

            output_vec.push(new_line);
        }

        for line in output_vec {
            println!("{}", line);
        }
    }
}