use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let player_tiles = make_a_set_of_seven_random_tiles();
    let subsets = produce_tile_subsets(player_tiles, 7);
    //do this properly
    tests();

    //create the dictionary
    let filename = "data";
    let lines = read_file(filename).unwrap();

    let dictionary = make_dictionary(lines);
    let results_list: Vec<String> = make_results_list(dictionary, subsets);
    let mut final_results_list = results_list.iter();

    while let Some(result) = final_results_list.next() {
        println!("{:?}", &result);
    }

    get_answers(results_list);
}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn make_a_set_of_seven_random_tiles() -> Vec<char> {
    let mut bag = Vec::new();
    const ASCIISET: &[u8] =
        b"aaaaaaaaaiiiiiiiiioooooooonnnnnnrrrrrrttttttllllssssuuuuddddgggbbccmmppffhhvvwwyykjxqz";
    for _ in 1..8 {
        let idx = rand::thread_rng().gen_range(0..ASCIISET.len());
        bag.push(ASCIISET[idx] as char)
    }
    bag.sort_by(|a, b| a.cmp(b));
    println!("The bag is {:?}", bag);
    return bag;
}

fn score_word(word: String) -> usize {
    let mut score: usize = 0;

    for letter in word.chars() {
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

fn produce_tile_subsets(player_tiles: Vec<char>, number_tiles: u8) -> Vec<String> {
    let mut subsets: Vec<String> = Vec::new();

    // Generate all possible subsets using bitwise operations
    for n in 0..1 << number_tiles {
        let mut subset = String::new();
        for (idx, tile) in player_tiles.iter().enumerate() {
            if n & (1 << idx) != 0 {
                subset.push(*tile);
            }
        }
        if !subset.is_empty() {
            subsets.push(subset);
        }
    }

    subsets.sort(); // Sort subsets lexicographically
    subsets
}
// fn produce_tile_subsets(player_tiles: Vec<char>, number_tiles: u8) -> Vec<String> {
//     let mut subsets: Vec<String> = Vec::new();
//     let mut binary_bytes: Vec<String> = Vec::new();
//     for n in 0..2_u8.pow(number_tiles.into()) {
//         binary_bytes.push(format!("{n:07b}"));
//     }
//
//     let mut binary_bits: Vec<Vec<char>> = Vec::new();
//
//     for word in binary_bytes.iter() {
//         binary_bits.push(word.chars().collect());
//     }
//
//     let mut buff = Vec::new();
//     for byte in binary_bits.iter() {
//         for (idx, bit) in byte.iter().enumerate() {
//             if *bit == '1' {
//                 buff.push(player_tiles[idx]);
//             }
//         }
//         if !buff.is_empty() {
//             buff.sort_by(|a, b| a.cmp(b));
//             let buff: &String = &buff.clone().into_iter().collect();
//             subsets.push(buff.to_string());
//         }
//         buff.clear();
//     }
//     return subsets;
// }

fn tests() {
    assert_eq!(
        produce_tile_subsets(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], 7),
        vec![
            "a", "ab", "abc", "abcd", "abcde", "abcdef", "abcdefg", "abcdeg", "abcdf", "abcdfg",
            "abcdg", "abce", "abcef", "abcefg", "abceg", "abcf", "abcfg", "abcg", "abd", "abde",
            "abdef", "abdefg", "abdeg", "abdf", "abdfg", "abdg", "abe", "abef", "abefg", "abeg",
            "abf", "abfg", "abg", "ac", "acd", "acde", "acdef", "acdefg", "acdeg", "acdf", "acdfg",
            "acdg", "ace", "acef", "acefg", "aceg", "acf", "acfg", "acg", "ad", "ade", "adef",
            "adefg", "adeg", "adf", "adfg", "adg", "ae", "aef", "aefg", "aeg", "af", "afg", "ag",
            "b", "bc", "bcd", "bcde", "bcdef", "bcdefg", "bcdeg", "bcdf", "bcdfg", "bcdg", "bce",
            "bcef", "bcefg", "bceg", "bcf", "bcfg", "bcg", "bd", "bde", "bdef", "bdefg", "bdeg",
            "bdf", "bdfg", "bdg", "be", "bef", "befg", "beg", "bf", "bfg", "bg", "c", "cd", "cde",
            "cdef", "cdefg", "cdeg", "cdf", "cdfg", "cdg", "ce", "cef", "cefg", "ceg", "cf", "cfg",
            "cg", "d", "de", "def", "defg", "deg", "df", "dfg", "dg", "e", "ef", "efg", "eg", "f",
            "fg", "g"
        ]
    );
}

fn make_dictionary(lines: Vec<String>) -> HashMap<String, String> {
    let mut dictionary: HashMap<String, String> = HashMap::new();

    for line in lines.iter() {
        let mut word: Vec<char> = line.trim_end().chars().collect();
        word.sort();
        let alpabeticised_word: String = word.into_iter().collect();
        dictionary.insert(line.trim().to_string(), alpabeticised_word.to_string());
    }
    return dictionary;
}

fn make_results_list(dictionary: HashMap<String, String>, subsets: Vec<String>) -> Vec<String> {
    let mut results_list: Vec<String> = Vec::new();
    for (key, value) in dictionary {
        if subsets.contains(&value.to_string()) {
            results_list.push(key);
        }
    }
    results_list
}

fn get_answers(results_list: Vec<String>) {
    let mut answer: Vec<(usize, &str)> = Vec::new();
    for result in results_list.iter() {
        if !results_list.is_empty() {
            answer.push((score_word(result.to_string()), result));
        } else {
            println!("Results list is empty");
            answer.push((0, "0"));
        }
    }
    answer.sort();
    if answer.is_empty() {
        println!("There is no scoring word");
    } else {
        println!("Top scoring word {:?}", answer.pop().unwrap());
    }
}
