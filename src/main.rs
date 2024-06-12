use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
//get rid of comments
//bug: only finding one occurence of combination, probably
//due to the hashmap

fn random_ascii() -> String {
    let mut bag = String::new();
    const ASCIISET: &[u8] =
        b"aaaaaaaaaiiiiiiiiioooooooonnnnnnrrrrrrttttttllllssssuuuuddddgggbbccmmppffhhvvwwyykjxqz";
    for _ in 1..8 {
        let idx = rand::thread_rng().gen_range(0..ASCIISET.len());
        bag.push(ASCIISET[idx] as char)
    }
    println!("The bag is {}", bag);
    //simply return bag here or placeholder "decimal".to_string()
    //return "decimal".to_string();
    return bag;
}

fn main() {
    let set = random_ascii();
    let sets: Vec<char> = set.chars().collect();

    //is there a way just to directly work
    //with binary or bools without using strings?
    let mut binary_bytes: Vec<String> = Vec::new();
    for n in 0..2_u32.pow(7) {
        //the value 7 here for 7 letters
        //how to insert a set_size variable into the formatter
        //else can I just compose a string to replace the {}
        binary_bytes.push(format!("{n:07b}"));
    }

    let mut binary_bits: Vec<Vec<char>> = Vec::new();

    for word in binary_bytes.iter() {
        binary_bits.push(word.chars().collect());
    }

    //println!("{:?}", &binary_bits);
    //assert_eq!(sets, ['a', 'b', 'c']);
    //println!("{:?}", &set);

    let mut subsets: Vec<String> = Vec::new();

    let mut buff = String::new();
    for byte in binary_bits.iter() {
        for (idx, bit) in byte.iter().enumerate() {
            if *bit == '1' {
                buff.push(sets[idx]);
            }
        }
        if !buff.is_empty() {
            subsets.push(buff.clone());
        }
        buff.clear();
    }
    subsets.sort();

    fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    let filename = "data";
    let lines = read_file(filename).unwrap();
    //    println!("{:?}", lines);

    //https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut dictionary: HashMap<String, String> = HashMap::new();

    for line in lines.iter() {
        let mut word: Vec<char> = line.trim_end().chars().collect();
        word.sort();
        let alpabeticised_word: String = word.into_iter().collect();
        dictionary.insert(line.trim().to_string(), alpabeticised_word);
    }

    //there is a bug missing anagrams
    //the algo will find decimal but not claimed for example
    let mut results_list: Vec<String> = Vec::new();
    for key in subsets.iter() {
        match dictionary.get(key) {
            Some(value) => {
                println!("{key}: {value}");
                results_list.push(key.clone());
            }
            None => continue,
        }
    }

    for result in results_list.iter() {
        println!("{}", score_word(result.to_string()));
    }

    println!("Results list {:?}", results_list);

    fn score_word(word: String) -> usize {
        //this repeats an earlier process
        let mut letters: Vec<char> = Vec::new();
        let mut score: usize = 0;

        for letter in word.chars() {
            letters.push(letter);
        }

        for letter in letters.iter() {
            match letter {
                'e' | 'a' | 'i' | 'o' | 'n' | 'r' | 't' | 'l' | 's' | 'u' => score += 1,
                'd' | 'g' => score += 2,
                'b' | 'c' | 'm' | 'p' => score += 3,
                'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
                'k' => score += 5,
                'j' | 'x' => score += 8,
                'q' | 'z' => score += 10,
                _ => println!("missed a case"),
            }
        }
        return score;
    }
}
