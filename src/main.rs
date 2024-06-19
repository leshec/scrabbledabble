use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//Tidy up a lot.
//put functions, variables etc in sensible locations
//some variables should be structs e.g. result word and score are coupled
//use appropriate functions
//simply the subsets building, too mnay allocations/types
//best way to read file and ingest/process?
//add tests

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

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

fn main() {
    let set = random_ascii();
    //let set = "decimal"; //for testing purposes
    println!("The starting bag is {}", set);
    let sets: Vec<char> = set.chars().collect();

    //how can i work more directly with binary?
    //can i just have arrary of bools or read u8 as bytes/bits
    let mut binary_bytes: Vec<String> = Vec::new();
    for n in 0..2_u32.pow(7) {
        //could generalise the input 7 etc
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

    let mut buff = Vec::new();
    for byte in binary_bits.iter() {
        for (idx, bit) in byte.iter().enumerate() {
            if *bit == '1' {
                buff.push(sets[idx]);
            }
        }
        if !buff.is_empty() {
            buff.sort_by(|a, b| a.cmp(b));
            let buff: &String = &buff.clone().into_iter().collect();
            subsets.push(buff.to_string());
        }
        buff.clear();
    }

    let filename = "data";
    let lines = read_file(filename).unwrap();
    //    println!("{:?}", lines);

    let mut dictionary: HashMap<String, String> = HashMap::new();

    for line in lines.iter() {
        let mut word: Vec<char> = line.trim_end().chars().collect();
        word.sort();
        let alpabeticised_word: String = word.into_iter().collect();
        dictionary.insert(line.trim().to_string(), alpabeticised_word);
    }

    //Finding words that match subsets of tiles
    let mut results_list: Vec<String> = Vec::new();
    for (key, value) in dictionary {
        if subsets.contains(&value) {
            results_list.push(key.clone());
        }
    }

    let mut answer: Vec<(usize, String)> = Vec::new();
    for result in results_list.iter() {
        answer.push((score_word(result.clone()), result.clone()));
    }
    answer.sort();
    // println!("The answer is {:#?}", answer);
    println!("The list of words are {:?}", &results_list);
    println!(
        "Among the top five scoring words are {:?}{:?}{:?}{:?}{:?}",
        answer.pop().unwrap(),
        answer.pop().unwrap(),
        answer.pop().unwrap(),
        answer.pop().unwrap(),
        answer.pop().unwrap()
    );
}
