fn main() {
    //convert file to haskset

    let set = String::from("abcdefg");
    let set_size: usize = set.len();
    let sets: Vec<char> = set.chars().collect();

    //is there a way just to directly work
    //with binary or bools without using strings?
    let mut binary_list: Vec<String> = Vec::new();
    for n in 0..2_u32.pow(set_size.try_into().unwrap()) {
        //the value 7 here for 7 letters
        //how to insert a set_size into the variable?
        binary_list.push(format!("{n:07b}"));
    }

    let mut binary_vec: Vec<Vec<char>> = Vec::new();

    for word in binary_list.iter() {
        binary_vec.push(word.chars().collect());
    }

    //println!("{:?}", &binary_vec);
    //assert_eq!(sets, ['a', 'b', 'c']);
    //println!("{:?}", &set);

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
    subsets.sort();

    println!("{:?}", subsets);
}
