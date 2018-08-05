// Copyright (c) 2018 Jason Graalum & Nathan Reed
// Crate which defines a hierarchical tree of generic objects
//

use std::fmt::Debug;
use self::BST::*;
///  Generic Search Tree
///

enum BST<V> {
    Empty,
    NonEmpty(Box<BST_node<V>>),
}

///
///
///
///
///
///
/// A BST is an implementation of a SearchTree
struct BST_node<V> {
    val: V,
    right : BST<V>,
    left : BST<V>,
    depth : usize,
}

///BinTreeIter
///
struct BinTreeIter <'a, V: 'a>{
    iter_stack: Vec<&'a BST_node<V>>
}

pub struct IntoIter_Post<V>(BST_node<V>);


impl <'a, V: 'a> BinTreeIter<'a, V> {
    fn push_leftmost (&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, V> Iterator for BinTreeIter <'a, V> {
    type Item = &'a V;
    fn next(& mut self) -> Option<&'a V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_leftmost(&node.right);

        Some(&node.val)
    }
}

/*
impl<V> Iterator for IntoIter<V> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
    }
}
*/

impl <V> BST<V>
    where V : Debug + Copy + Clone + Ord + PartialEq {

    fn new() -> Self {
        Empty
    }

    fn iter(& self) -> BinTreeIter<V> {
        let mut iter = BinTreeIter { iter_stack: Vec::new()};
        iter.push_leftmost(self);
        iter
    }

    /// https://gist.github.com/aidanhs  Binary Search Tree Tutorial
    fn insert(&mut self, new_val: V) {

        match self {

            & mut Empty => {
                let new_tree = NonEmpty(Box::new(BST_node {left: Empty, right: Empty, val: new_val, depth: 1}));
                *self = new_tree;
            },
            & mut NonEmpty(ref mut n) => {
                if n.val == new_val {
                    return;
                }
                let target_subtree = if n.val > new_val { &mut n.left } else { &mut n.right };
                match target_subtree {
                    &mut NonEmpty (_) => target_subtree.insert(new_val),
                    &mut Empty => {
                        let boxed_node = NonEmpty(Box::new(BST_node{left: Empty, right: Empty,  val: new_val, depth: 1}));
                        *target_subtree = boxed_node;
                    }
                }
            }
        }
    }

    fn peek(&self) -> Option<&V> {
        match self {
            &Empty => None, 
            &NonEmpty(ref n) => Some(&n.val),
        }
    }

}

#[test]
fn add_node() {
    let mut tree : BST<i32> = BST::new();
    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);
    assert_eq!(tree.peek(), Some(&8));
}

#[test]
fn iterator_test (){
    let mut tree : BST<i32> = BST::new();
    let mut node_vec: Vec<i32> = vec![1,6,7,8,10,13,20];
    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);
    let vec_reverse = node_vec.reverse();
    for node in tree.iter() {

        assert_eq!(node,  &node_vec.pop().unwrap());
    }
}

