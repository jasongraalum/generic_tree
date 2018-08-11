// Copyright (c) 2018 Jason Graalum & Nathan Reed // Crate which defines a hierarchical tree of generic objects
//

use std::fmt::Debug;
use self::BST::*;
//
///  Generic Search Tree
///

enum BST<V> {
    Empty,
    NonEmpty(Box<BSTNode<V>>),
}


///
///
/// A BST is an implementation of a SearchTree
struct BSTNode<V> {
    val: V,
    right : BST<V>,
    left : BST<V>,
    depth : usize,
}

///BinTreeIter
///
///
struct BSTPostIter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<(&'a BSTNode<V>, bool)>
}

impl <'a, V: 'a> BSTPostIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_leftmost_thenright (&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            println!("Tree: {:?}",node.val);
            self.iter_stack.push((node, false));
            // If left, set tree to left else if right, set tree to right
            match (&node.left, &node.right) {
                (&Empty, &NonEmpty(_)) => tree = &node.right,
                (_,_) => tree = &node.left,
            }
        }
    }

    fn push_node(&mut self, node : &'a BSTNode<V>) {
        println!("Node: {:?}",node.val);
        self.iter_stack.push((node, true));
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BSTPostIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = &'a V;
    fn next(&mut self) -> Option<&'a V> {
        let (node, _) = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };
        match self.iter_stack.pop() {
            Some((n,v)) =>  {
                self.push_node(&n);
                if v == false {
                    self.push_leftmost_thenright(&n.right);
                }
            },
            None => return None,
        }

        println!("Popping: {:?}", &node.val);
        Some(&node.val)
    }
}


struct BSTPreIter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<&'a BSTNode<V>>
}

impl <'a, V: 'a> BSTPreIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_top (&mut self, mut tree: &'a BST<V>) {
        if let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
        }
    }
}
// Iterator for Post-Order
impl<'a, V> Iterator for BSTPreIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
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

struct BSTInOrderIter<'a, V: 'a> where V : Debug + Copy + Clone + Ord + PartialEq {
    iter_stack: Vec<&'a BSTNode<V>>
}

impl <'a, V: 'a> BSTInOrderIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    fn push_leftmost (&mut self, mut tree: &'a BST<V>) {
        while let NonEmpty(ref node) = *tree {
            self.iter_stack.push(node);
            tree = &node.left;
        }
    }
}
// Iterator for In-Order
impl<'a, V> Iterator for BSTInOrderIter<'a, V> where V : Debug + Copy + Clone + Ord + PartialEq {
    type Item = &'a V;
    // pop top of stack and return value, push left and then right nodes if they exist
    fn next(&mut self) -> Option<&'a V> {
        let node = match self.iter_stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_leftmost(&node.right);

        Some(&node.val)
    }
}



//++++++++++++++++++++++++++++++++++IMPL-BST+++++++++++++++++++++++++++++++++++++

impl <'a, V> BST <V>
    where V : Debug + Copy + Clone + Ord + PartialEq {

    fn new() -> Self {
        Empty
    }

    fn iter_post_order(& self) -> BSTPostIter<V> {
        let mut iter = BSTPostIter { iter_stack: Vec::new()};
        iter.push_leftmost_thenright(self);
        iter
    }

    // Pushed reference to top node
    fn iter_pre_order(& self) -> BSTPreIter<V> {
        let mut iter = BSTPreIter { iter_stack: Vec::new()};
        iter.push_top(self);
        iter
    }
    // Pushed reference to top node
    fn iter_in_order(& self) -> BSTInOrderIter<V> {
        let mut iter = BSTInOrderIter { iter_stack: Vec::new()};
        iter.push_leftmost(self);
        iter
    }

    /// https://gist.github.com/aidanhs  Binary Search Tree Tutorial
    fn insert(&mut self, new_val: V) {
        match self {
            &mut Empty => {
                let new_tree = NonEmpty(Box::new(BSTNode {left: Empty, right: Empty, val: new_val, depth: 1}));
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
                        let boxed_node = NonEmpty(Box::new(BSTNode {left: Empty, right: Empty,  val: new_val, depth: 1}));
                        *target_subtree = boxed_node;
                    }
                }
            }
        }
    }

    pub fn depth() -> u64
    {
        unimplemented!()
    }

    pub fn size() -> u64
    {

    }
    pub fn find() -> Option<&V>
    {
        unimplemented!()
    }

    pub fn contains() -> bool
    {
        unimplemented!()
    }

    pub fn remove() -> bool
    {
        unimplemented!()
    }

    pub fn merge(other_tree : BST<V>)
    {
        unimplemented!()
    }

    pub fn print_in_order()
    {
        unimplemented!()
    }
    pub fn print_post_order()
    {
        unimplemented!()
    }
    pub fn print_pre_order()
    {
        unimplemented!()
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
        let mut node_vec: Vec<i32> = vec![1,7,6,10,20,13,8];
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

#[test]
fn in_order_iterator_test (){
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
        for node in tree.iter_in_order() {
            assert_eq!(node, &node_vec.pop().unwrap());
        }
    }
}
