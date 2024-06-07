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
    let mut root = Node {
        prev: '0',
        value: '1',
        children: Vec::new(),
    };

    root.add_node('1', 'a');
    root.add_node('1', 'b');
    root.add_node('1', 'c');

    for node in root.children.iter_mut() {
        if node.prev == '1' && node.value == 'a' {
            node.add_node(node.prev, 'z');
            node.add_node(node.prev, 'y');
            node.add_node(node.prev, 'x');
            println!("{:?}", node);
        }
    }

    println!("*****************************************");
    println!("the tree is {:#?}", &root);
}

//save this for now
