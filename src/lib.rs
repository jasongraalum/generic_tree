// Copyright (c) 2018 Jason Graalum & Nathan Reed // Crate which defines a hierarchical tree of generic objects
//

use std::fmt::Debug;
use self::BST::*;
use std::mem;
//
///  Generic Search Tree
///
#[allow(dead_code)]
enum BST<V> {
    Empty,
    NonEmpty(Box<BSTNode<V>>),
}

///
///
/// A BST is an implementation of a SearchTree
#[allow(unused_variables)]
#[allow(dead_code)]
struct BSTNode<V> {
    val: Option<V>,
    right: BST<V>,
    left: BST<V>,
    depth: usize,
}

pub struct IntoIter<V>(BST<V>);

///
///BinTreeIter
///
struct BSTPostIter<'a, V: 'a>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    iter_stack: Vec<(&'a BSTNode<V>, bool)>,
}

impl<'a, V: 'a> BSTPostIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    fn push_leftmost_thenright(&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push((node, false));
            // If left, set tree to left else if right, set tree to right
            match (&node.left, &node.right) {
                (&Empty, &NonEmpty(_)) => tree = &node.right,
                (_, _) => tree = &node.left,
            }
        }
    }

    fn push_node(&mut self, node: &'a BSTNode<V>) {
        self.iter_stack.push((node, true));
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BSTPostIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> {
        let (node, _) = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };
        match self.iter_stack.pop() {
            Some((n, v)) => {
                self.push_node(&n);
                if !v {
                    self.push_leftmost_thenright(&n.right);
                }
            }
            None => return None,
        }

        match node.val {
            None => None,
            Some(ref v) => Some(&v),
        }
    }
}

struct BSTPreIter<'a, V: 'a>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    iter_stack: Vec<&'a BSTNode<V>>,
}

impl<'a, V: 'a> BSTPreIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    fn push_top(&mut self, tree: &'a BST<V>) {
        if let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
        }
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BSTPreIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    type Item = &'a V;
    // pop top of stack and return value, push left and then right nodes if they exist
    fn next(&mut self) -> Option<&'a V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        match (&node.left, &node.right) {
            (&NonEmpty(_), &NonEmpty(_)) => {
                self.push_top(&node.right);
                self.push_top(&node.left);
            }
            (&NonEmpty(_), _) => {
                self.push_top(&node.left);
            }
            (_, &NonEmpty(_)) => {
                self.push_top(&node.right);
            }
            (_, _) => {}
        }

        match node.val {
            None => None,
            Some(ref v) => Some(&v),
        }
    }
}

struct BSTInOrderIter<'a, V: 'a>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    iter_stack: Vec<&'a BSTNode<V>>,
}

impl<'a, V: 'a> BSTInOrderIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    fn push_leftmost(&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
            tree = &node.left;
        }
    }
}
// Iterator for In-Order
impl<'a, V> Iterator for BSTInOrderIter<'a, V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    type Item = &'a V;
    // pop top of stack and return value, push left and then right nodes if they exist
    fn next(&mut self) -> Option<&'a V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_leftmost(&node.right);

        match node.val {
            None => None,
            Some(ref v) => Some(&v),
        }
    }
}

//++++++++++++++++++++++++++++++++++IMPL-BST+++++++++++++++++++++++++++++++++++++
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a, V> BST<V>
where
    V: Debug + Copy + Clone + Ord + PartialEq,
{
    fn new() -> Self {
        Empty
    }

    pub fn iter_post_order(&self) -> BSTPostIter<V> {
        let mut iter = BSTPostIter {
            iter_stack: Vec::new(),
        };
        iter.push_leftmost_thenright(self);
        iter
    }

    // Pushed reference to top node
    pub fn iter_pre_order(&self) -> BSTPreIter<V> {
        let mut iter = BSTPreIter {
            iter_stack: Vec::new(),
        };
        iter.push_top(self);
        iter
    }
    // Pushed reference to top node
    pub fn iter_in_order(&self) -> BSTInOrderIter<V> {
        let mut iter = BSTInOrderIter {
            iter_stack: Vec::new(),
        };
        iter.push_leftmost(self);
        iter
    }

    pub fn into_iter(self) -> IntoIter<V> {
        IntoIter(self)
    }

    /// https://gist.github.com/aidanhs  Binary Search Tree Tutorial
    /// Modified
    pub fn insert(&mut self, new_val: V) {
        match *self {
            Empty => {
                let new_tree = NonEmpty(Box::new(BSTNode {
                    left: Empty,
                    right: Empty,
                    val: Some(new_val),
                    depth: 1,
                }));
                *self = new_tree;
            }
            NonEmpty(ref mut n) => match n.val {
                None => return,
                Some(v) => {
                    if v == new_val {
                        return;
                    }
                    let target_subtree = if v > new_val {
                        &mut n.left
                    } else {
                        &mut n.right
                    };
                    match *target_subtree {
                        NonEmpty(_) => target_subtree.insert(new_val),
                        Empty => {
                            let boxed_node = NonEmpty(Box::new(BSTNode {
                                left: Empty,
                                right: Empty,
                                val: Some(new_val),
                                depth: 1,
                            }));
                            *target_subtree = boxed_node;
                        }
                    }
                }
            },
        }
    }

    pub fn depth() -> u64 {
        unimplemented!()
    }

    pub fn size() -> u64 {
        unimplemented!()
    }
    pub fn find() -> Option<&'a V> {
        unimplemented!()
    }

    pub fn contains() -> bool {
        unimplemented!()
    }

    // Swap values of the current BST with the left node BST
    // Return the current BST
    pub fn swap_right(&mut self) -> Option<&BST<V>> {
        let mut curr_val: Option<V> = None;
        let mut temp_val: Option<V> = None;

        match *self {
            Empty => return None,
            NonEmpty(ref mut node) => {
                mem::swap(&mut curr_val, &mut node.val);
                match node.right {
                    Empty => return None,
                    NonEmpty(ref mut r) => {
                        mem::swap(&mut temp_val, &mut r.val);
                        mem::swap(&mut curr_val, &mut r.val);
                    }
                };
                mem::swap(&mut temp_val, &mut node.val);
            }
        };

        Some(self)
    }

    // Swap values of the current BST with the left node BST
    // Return the current BST
    pub fn swap_left(&mut self) -> Option<&BST<V>> {
        let mut curr_val: Option<V> = None;
        let mut temp_val: Option<V> = None;

        match *self {
            Empty => return None,
            NonEmpty(ref mut node) => {
                mem::swap(&mut curr_val, &mut node.val);
                match node.left {
                    Empty => return None,
                    NonEmpty(ref mut r) => {
                        mem::swap(&mut temp_val, &mut r.val);
                        mem::swap(&mut curr_val, &mut r.val);
                    }
                };
                mem::swap(&mut temp_val, &mut node.val);
            }
        };

        Some(self)
    }

    pub fn take_right(&mut self) -> Option<BST<V>> {
        match *self {
            Empty => None,
            NonEmpty(ref mut tree) => {
                let mut right: BST<V> = Empty;
                mem::swap(&mut tree.right, &mut right);
                Some(right)
            }
        }
    }

    pub fn take_left(&mut self) -> Option<BST<V>> {
        match *self {
            Empty => None,
            NonEmpty(ref mut tree) => {
                let mut left: BST<V> = Empty;
                mem::swap(&mut tree.left, &mut left);
                Some(left)
            }
        }
    }

    pub fn merge(other_tree: BST<V>) {
        unimplemented!()
    }

    pub fn print_in_order() {
        unimplemented!()
    }
    pub fn print_post_order() {
        unimplemented!()
    }
    pub fn print_pre_order() {
        unimplemented!()
    }

    fn peek(&self) -> Option<&V> {
        match *self {
            Empty => None,
            NonEmpty(ref n) => match n.val {
                None => None,
                Some(ref v) => Some(&v),
            },
        }
    }
}

#[test]
fn add_node() {
    let mut tree: BST<i32> = BST::new();
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
fn post_iterator_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for i in 0..3 {
        let mut node_vec: Vec<i32> = vec![1, 7, 6, 10, 20, 13, 8];
        let vec_reverse = node_vec.reverse();
        for node in tree.iter_post_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}
#[test]
fn pre_iterator_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for i in 0..3 {
        let mut node_vec: Vec<i32> = vec![8, 6, 1, 7, 13, 10, 20];
        let vec_reverse = node_vec.reverse();
        for node in tree.iter_pre_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}

#[test]
fn in_order_iterator_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    for i in 0..3 {
        let mut node_vec: Vec<i32> = vec![1, 6, 7, 8, 10, 13, 20];
        let vec_reverse = node_vec.reverse();
        for node in tree.iter_in_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}
#[test]
fn swap_right_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    let mut node_vec: Vec<i32> = vec![1, 6, 7, 13, 10, 8, 20];
    let vec_reverse = node_vec.reverse();

    tree.swap_right();

    for node in tree.iter_in_order() {
        assert_eq!(node, &node_vec.pop().unwrap());
    }
}
#[test]
fn swap_left_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    let mut node_vec: Vec<i32> = vec![1, 8, 7, 6, 10, 13, 20];
    let vec_reverse = node_vec.reverse();

    tree.swap_left();

    for node in tree.iter_in_order() {
        assert_eq!(node, &node_vec.pop().unwrap());
    }
}

#[test]
fn take_left_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    let mut left_node_vec: Vec<i32> = vec![1, 6, 7];
    let mut right_node_vec: Vec<i32> = vec![8, 10, 13, 20];
    let left_vec_reverse = left_node_vec.reverse();
    let right_vec_reverse = right_node_vec.reverse();

    if let Some(left_tree) = tree.take_left() {
        for node in left_tree.iter_in_order() {
            assert_eq!(node, &left_node_vec.pop().unwrap());
        }
        for node in tree.iter_in_order() {
            assert_eq!(node, &right_node_vec.pop().unwrap());
        }
    }
}
#[test]
fn take_right_test() {
    let mut tree: BST<i32> = BST::new();

    tree.insert(8);
    tree.insert(13);
    tree.insert(6);
    tree.insert(1);
    tree.insert(20);
    tree.insert(10);
    tree.insert(7);

    let mut left_node_vec: Vec<i32> = vec![1, 6, 7, 8];
    let mut right_node_vec: Vec<i32> = vec![10, 13, 20];
    let left_vec_reverse = left_node_vec.reverse();
    let right_vec_reverse = right_node_vec.reverse();

    if let Some(right_tree) = tree.take_right() {
        for node in right_tree.iter_in_order() {
            assert_eq!(node, &right_node_vec.pop().unwrap());
        }
        for node in tree.iter_in_order() {
            assert_eq!(node, &left_node_vec.pop().unwrap());
        }
    }
}
