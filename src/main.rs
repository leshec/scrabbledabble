fn main() {
    let set = String::from("abc");
    let sets: Vec<char> = set.chars().collect();

    let mut binary_list: Vec<String> = Vec::new();
    for n in 0..8 {
        binary_list.push(format!("{n:03b}"));
    }

    let mut binary_vec: Vec<Vec<char>> = Vec::new();

    for word in binary_list.iter() {
        binary_vec.push(word.chars().collect());
    }

    println!("{:?}", &binary_vec);
    assert_eq!(sets, ['a', 'b', 'c']);
    println!("{:?}", &set);

    let mut subsets: Vec<String> = Vec::new();

    let mut buff = String::new();
    for item in binary_vec.iter() {
        for (idx, value) in item.iter().enumerate() {
            if *value == '1' {
                buff.push(sets[idx]);
            }
        }
        if !buff.is_empty() {
            subsets.push(buff.clone());
        }
        buff.clear();
    }

    println!("{:?}", subsets);
}
