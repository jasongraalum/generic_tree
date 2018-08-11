// Copyright (c) 2018 Jason Graalum & Nathan Reed
// Crate which defines a hierarchical tree of generic objects
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
struct BST_iter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<&'a BST_node<V>>
}

impl <'a, V: 'a> BST_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_leftmost (&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
            tree = &node.left;
        }
    }
}

struct BST_into_iter<V> where V : Debug + Copy + Clone + Ord + PartialEq {
    top : BST<V>,
    next_node : Option<BST_node<V>>,
}

// IntoIterator Post Order Traversal
impl<V> BST_into_iter<V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn set_leftmost (mut self, mut tree: BST<V>) {

        if let BST::NonEmpty(mut root) = tree {
            match (*root.left, *root.right) {
                (Empty, Empty) => {
                    self.next_node = Some(*root);
                },
                (NonEmpty(l), _) => self.set_leftmost(l),
                (Empty,NonEmpty(r)) => self.set_leftmost(r),
            }
        }
        // If no left or right, push node
        // If left, call set_leftmost on left
        // else call set_leftmost on right
        //mem::swap(&mut right, &mut top.right);
    }
}

// Iterator for Post-Order
impl<'a, V> Iterator for BST_iter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
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

impl IntoIterator for BST<V> {
    type Item = V;
    type IntoIter = BST::IntoIter<V>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.

    }
}
// Into Iterator for Post-Order
impl<V> Iterator for BST_into_iter<V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = V;
    fn next(mut self) -> Option<V> {
        let node = match self.next_node {
            Some(n) => n,
            None => return None,
        };

        // Need to restart set_leftmost from the top of the tree
        self.set_leftmost(self.top);

        Some(node.val)
    }
}

//++++++++++++++++++++++++++++++ITER_MUT++++++++++++++++++++++++++++++++++++++++
///BinTreeIter_MUTABLE
///
///
/*
struct BST_iter_mut<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec< &'a mut BST_node<V>>
}

impl <'a, V: 'a> BST_iter_mut<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_leftmost (&mut self, mut tree: &'a mut BST<V>) {
        while let NonEmpty(ref mut node) = *tree {
            self.iter_stack.push(node);
            tree = & mut node.left;
        }
    }
}

impl<'a, V> Iterator for BST_iter_mut<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<&'a mut V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_leftmost(& mut node.right);

        Some(& mut node.val)
    }
}
*/


//++++++++++++++++++++++++++++++++++IMPL-BST+++++++++++++++++++++++++++++++++++++

impl <'a, V> BST <V>
    where V : Debug + Copy + Clone + Ord + PartialEq {

    fn new() -> Self {
        Empty
    }

    fn iter(& self) -> BST_iter<V> {
        let mut iter = BST_iter { iter_stack: Vec::new()};
        iter.push_leftmost(self);
        iter
    }

    fn into_iter(self) -> BST_into_iter<V> {
        let mut iter = BST_into_iter { next_node : None, top : self };
        iter.set_leftmost(self);
        iter
    }
    /*
    fn iter_mut(&mut self) -> BST_iter_mut<V> {
        let mut iter = BST_iter_mut { iter_stack: Vec::new()};
        iter.push_leftmost(self);
        iter
    }
    */

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
fn iterator_test (){
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
        for node in tree.iter() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}

#[test]
fn into_iterator_testCHECK_EMPTY () {
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
        for node in tree.into_iter() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}

#[test]
fn into_iterator_test_consume () {
    let mut tree : BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for node in tree.into_iter() {
    }

    match tree {
        Empty => assert!(true),
        NotEmpty => assert!(false),
    }

}

