// Copyright (c) 2018 Jason Graalum & Nathan Reed // Crate which defines a hierarchical tree of generic objects
//

use std::fmt::Debug;
use self::BST::*;
use std::mem;
///  Generic Search Tree
///

enum BST<V> {
    Empty,
    NonEmpty(Box<BST_node<V>>),
}

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
///
struct BST_post_iter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<&'a BST_node<V>>
}

impl <'a, V: 'a> BST_post_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_leftmost (&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
            tree = &node.left;
        }
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BST_post_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_leftmost(&node.right);

        Some(&node.val)
    }
}


struct BST_pre_iter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<&'a BST_node<V>>
}

impl <'a, V: 'a> BST_pre_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_top (&mut self, mut tree: &'a BST<V>) {
        if let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
        }
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BST_pre_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = &'a V;
    // pop top of stack and return value, push left and then right nodes if they exist
    fn next(&mut self) -> Option<&'a V> {

        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        match (&node.left, &node.right) {
            (&NonEmpty(_), &NonEmpty(_) ) => {
                self.push_top(&node.right);
                self.push_top(&node.left);
            }
            (&NonEmpty(_), _ ) => {
                self.push_top(&node.left);
            }
            (_, &NonEmpty(_) ) => {
                self.push_top(&node.right);
            }
            (_,_) => {},
        }

        Some(&node.val)
    }
}



//++++++++++++++++++++++++++++++++++IMPL-BST+++++++++++++++++++++++++++++++++++++

impl <'a, V> BST <V>
    where V : Debug + Copy + Clone + Ord + PartialEq {

    fn new() -> Self {
        Empty
    }

    fn iter_post_order(& self) -> BST_post_iter<V> {
        let mut iter = BST_post_iter { iter_stack: Vec::new()};
        iter.push_leftmost(self);
        iter
    }

    // Pushed reference to top node
    fn iter_pre_order(& self) -> BST_pre_iter<V> {
        let mut iter = BST_pre_iter { iter_stack: Vec::new()};
        iter.push_top(self);
        iter
    }

    /// https://gist.github.com/aidanhs  Binary Search Tree Tutorial
    fn insert(&mut self, new_val: V) {
        match self {
            &mut Empty => {
                let new_tree = NonEmpty(Box::new(BST_node {left: Empty, right: Empty, val: new_val, depth: 1}));
                *self = new_tree;
            },
            &mut NonEmpty(ref mut n) => {
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
fn post_iterator_test (){
    let mut tree : BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for i in 0..3 {
        let mut node_vec: Vec<i32> = vec![1,6,7,8,10,13,20];
        let vec_reverse = node_vec.reverse();
        for node in tree.iter_post_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}
#[test]
fn pre_iterator_test (){
    let mut tree : BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for i in 0..3 {
        let mut node_vec: Vec<i32> = vec![8,6,1,7,13,10,20];
        let vec_reverse = node_vec.reverse();
        for node in tree.iter_pre_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}

