use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//get rid of comments
//bug: only finding one occurence of combination, probably
//due to the hashmap

fn main() {
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
        dictionary.insert(line.to_string(), alpabeticised_word);
    }

    //println!("{:?}", dictionary);
    //autogenerate this later...
    //add scores etc and weighted distribution
    let set = String::from("decimal");
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

    let mut results_list: Vec<String> = Vec::new();

    //println!("{:?}", subsets);
    // Look up the values associated with some keys.
    //probably need tomake this a function and call more than once
    //as there is a bug missing anagrams
    //the algo will find decimal but not claimed for example
    for key in subsets.iter() {
        match dictionary.get(key) {
            Some(value) => {
                println!("{key}: {value}");
                results_list.push(key.clone());
            }
            None => continue,
        }
    }

    println!("Results list {:?}", results_list);

    //Ok so I can get the word i want from dictinary the key above.
    //stick keys in a list, score them and take the highest score
    //in this case simply longest

    //add some asserts
    //need to generate tiles from weighted distro e.g:
    //     SEE NOTES below...
    //     - 1 point: E, A, I, O, N, R, T, L, S, U
    // - 2 points: D, G
    // - 3 points: B, C, M, P
    // - 4 points: F, H, V, W, Y
    // - 5 points: K
    // - 8 points: J, X
    // - 10 points: Q, Z

    fn score_word(word: String) -> usize {
        //this repeats an earlier process
        let mut letters: Vec<char> = Vec::new();
        let mut score: usize = 0;

        for letter in word.chars() {
            letters.push(letter);
        }
        
        for letter in letters.iter(){
            match letter {
                
                E, A, I, O, N, R, T, L, S, U => score +=1,
        D, G => score +=2,
        B, C, M, P =>score +=3,
        F, H, V, W, Y => score +=4,
        K => score +=5,
        J, X => score +=8,
        Q, Z =>score +=10,
            }
        }
        //return letters
        return 1;
    }

    //
    //     - 12 tiles: E
    // - 9 tiles: A,I
    // - 8 tiles: O
    // - 6 tiles: N,R,T
    // - 4 tiles: L,S,U,D
    // - 3 tiles: G
    // - 2 tiles: B,C,M,P,F,H,V,W,Y
    // - 1 tiles: K,J,X,Q,Z
    //
    //     #[cfg(feature = "std")]
    // pub fn random_ascii() -> char {
    //     const ASCIISET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    //                             abcdefghijklmnopqrstuvwxyz\
    //                             0123456789)(*&^%$#@!~. ";
    //     let idx = rand::thread_rng().gen_range(0..ASCIISET.len());
    //     ASCIISET[idx] as char
    // }
}
