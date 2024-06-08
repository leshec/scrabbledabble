#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Node {
    prev: char,
    value: char,
    children: Vec<Node>,
}

impl Node {
    fn add_node(&mut self, parent: char, label: char) {
        let _ = &self.children.push(Node {
            prev: parent,
            value: label,
            children: Vec::new(),
        });
    }
}

fn main() {
    //user data will be 7 chars
    //get user input and check it
    //store in a vec<char>

    let user_input = vec!['a', 'b', 'c'];

    let mut root = Node {
        prev: '0',
        value: '1',
        children: Vec::new(),
    };

    // println!("view into tree****************************************");
    //    for node in root.children.iter_mut() {
    //       assert_ne!(node.prev, node.value);
    //     if node.prev == '1' && node.value == 'a' {
    //         node.add_node(node.value, 'x');
    //         node.add_node(node.value, 'y');
    //         node.add_node(node.value, 'x');
    //         //check duplication with set at some point
    //      println!("{:?}", node);
    // }
    // }
    //
    println!("insertion*****************************************");
    //develop fucntion or some recursion to go down the tree
    for letter in user_input.clone() {
        if root.children.is_empty() || root.value == '1' {
            root.add_node(root.value, letter);
        }
    }

    for node in root.children.iter_mut() {
        for letter in user_input.clone() {
            assert_ne!(node.prev, node.value);
            if node.value != letter {
                node.add_node(node.value, letter);
            }
        }
    }
    println!("view longform*****************************************");
    println!("the tree is {:#?}", &root);
    //impl a pretter print
    println!("*****************************************");
}

//save this for now
