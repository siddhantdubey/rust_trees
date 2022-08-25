/* 
 * Binary Search Tree Implementation in Rust
 * */
use std::cmp::Ord;
use std::cmp::Ordering;

pub struct Tree {
    head: Option<Box<Node>>,
}

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
} 

impl Default for Tree {
    fn default() -> Self{
        Self::new()
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { head: None}
    }

    pub fn add(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            val: elem,
            left: None,
            right: None,
        });
        let mut curr = &mut self.head;
        loop {
            match curr {
                Some(node) => {
                    if elem < node.val {
                        curr = &mut node.left;
                    } else {
                        curr = &mut node.right;
                    }
                }
                None => {
                    *curr = Some(new_node);
                    break;
                }
            }
        }
    }

    pub fn search(&mut self, elem: i32) -> bool {
        let mut curr = &mut self.head;
        loop {
            match curr {
                Some(node) => {
                    match elem.cmp(&node.val) {
                        Ordering::Less => {curr = &mut node.left},
                        Ordering::Greater => {curr = &mut node.right},
                        Ordering::Equal => {return true}
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::Tree;
    #[test]
    fn test_add() {
        let mut mut_tree = Tree::new();
        mut_tree.add(1);
        assert_eq!(mut_tree.head.as_ref().unwrap().val, 1);
        mut_tree.add(2);
        assert_eq!(mut_tree.head.as_ref().unwrap().right.as_ref().unwrap().val, 2);
    }
    #[test]
    fn test_search() {
        let mut mut_tree = Tree::new();
        mut_tree.add(1);
        mut_tree.add(2);
        mut_tree.add(3);
        mut_tree.add(5);
        let search_five = mut_tree.search(5);
        assert!(search_five);
        let search_seven = mut_tree.search(7);
        assert!(!search_seven);
    }

}
