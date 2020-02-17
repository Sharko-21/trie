use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;
use std::any::Any;
use std::io;
use std::fmt::Error;
use std::io::ErrorKind;

//#[derive(Debug)]
trait Something{}

#[derive(Debug)]
struct Node {
    label: char,
    value: Box<Any>,
    children: Vec<Node>,
    is_set: bool,
}


struct Trie {
    name: String
}

impl Node {
    fn new() -> Node {
        Node{ label: ' ', value: Box::new("".to_string()), children: vec![], is_set: false }
    }

    fn set(&mut self, key: String, value: Box<Any>) {
        let mut node = self;

        for (i, c) in key.chars().enumerate() {
            if node.children.len() == 0 {
                let mut children = &mut node.children;
                children.push( Node{label: c, value:Box::new("".to_string()), children: vec![], is_set: false});
                node = &mut node.children[0];
                if i == key.chars().count() - 1 {
                    node.value = value;
                    node.is_set = true;
                    return;
                }
                continue;
            }
            for (child_num, child) in node.children.iter().enumerate() {
                if c == child.label {
                    node = &mut node.children[child_num];
                    if i == key.chars().count() - 1 {
                        node.value = value;
                        node.is_set = true;
                        return;
                    }
                    break;
                }
                if child_num == node.children.len() - 1{
                    let mut children = &mut node.children;
                    children.push( Node{label: c, value:Box::new("".to_string()), children: vec![], is_set: false});
                    node = &mut node.children[child_num + 1];
                    break;
                }
            }
        }
    }

    fn get(&mut self, key:String) -> Option<&Box<Any>> {
        let mut node = self;

        for (i, c) in key.chars().enumerate() {
            if node.children.len() == 0 {
                return None
            }
            if i == key.chars().count() - 1 && node.children.len() == 0 {
                if !node.is_set {
                    return None
                }
                if c == node.label {
                    return Some(&node.value);
                }
                return None
            }
            for (child_num, child) in node.children.iter().enumerate() {
                if c == child.label {
                    if i == key.chars().count() - 1 {
                        if !node.children[child_num].is_set {
                            return None
                        }
                        return Some(&mut node.children[child_num].value);
                    }
                    node = &mut node.children[child_num];
                    break;
                }
                if child_num == node.children.len() - 1 {
                    return None
                }
            }
        }
        return None
    }
}

fn main() {
    let mut node = Node::new();

    node.set("first".to_string(), Box::new("value 1".to_string()));
    node.set("second".to_string(), Box::new("value 2".to_string()));
    node.set("first".to_string(), Box::new("value 3".to_string()));
    node.set("first".to_string(), Box::new("value 4".to_string()));
    node.set("one_more_key".to_string(), Box::new("value 5".to_string()));

    match node.get("second".to_string()) {
        Some(val) => {
            match val.downcast_ref::<String>() {
                Some(as_string) => {
                    println!("{}", as_string);
                }
                None => {
                    println!("Not string!!!");
                }
            }
        },
        _ => println!("Empty")
    }
}
