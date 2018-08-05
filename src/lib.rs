// Copyright (c) 2018 Jason Graalum //
// Crate which defines a hierarchical tree of generic objects
//

use std::fmt::Debug;
///  Generic Search Tree
///
///  Defines SearchTree trait which implements Iter, IntoIter, and IterMut
///  Also implements Debug, Clone/Copy, Eq/Partial, Ord
///
///  Example:  Binary Search Tree
///
///  A binary search tree would implement SearchTree with node owning
///  zero, one or two additional nodes. It would implement the methods of
///  SearchTree - insert, remove, slice, split, find, iter
///
///
///
///
//trait SearchTree<V> : Iterator + IntoIterator
trait SearchTree<V>
    where V : Debug + Copy + Clone + Ord + PartialEq {
    /// Return a newly created Tree
    fn new(val : V) -> Self;

    /// insert a node, which could be a full tree, to the current tree at the current level
    fn insert(&mut self, node_val: V);

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn remove(&mut self);

    /// Return the value of the current tree root
    fn get_val(&self) -> &V;

    fn into_iter_postorder(mut self) -> Self;

    /*
    fn into_iter_preorder(mut self) -> Self;
    fn into_iter_inorder(mut self) -> Self;
    */

    /*
    fn iter_postorder(&self) -> &Self;
    fn iter_preorder(&self) -> &Self;
    fn iter_inorder(&self) -> &Self;
    */

    /*
    fn iter_mut_postorder(&mut self) -> &mut Self;
    fn iter_mut_preorder(&mut self) -> &mut Self;
    fn iter_mut_inorder(&mut self) -> &mut Self;
    */
}
///
///
///
///
///
///
/// A BST is an implementation of a SearchTree
struct BST<V> {
    val: V,
    right : Option<Box<BST<V>>>,
    left : Option<Box<BST<V>>>,
    depth : usize,
}

pub struct IntoIter_Post<V>(BST<V>);

impl<V> BST<V> {
    pub fn into_iter_postorder(mut self) -> IntoIter_Post<V> {
        IntoIter_Post(self)
    }

    fn take_right(&mut self) -> Option<Box<Self>>
    {
        let mut new_right = self.right.take();
        self.right=None;
        return new_right;
    }

    fn take_left(&mut self) -> Option<Box<Self>>
    {
        let mut new_left = self.left.take();
        self.left=None;
        return new_left;
    }

    fn take_left_most(&mut self) -> Option<Box<Self>>
    {
        match (&mut self.left,&mut self.right) {
            (None, None) => self,
            (None, Some(mut r)) => { self.r = None; r.take_left_most() },
            (Some(mut l),_) => { self.left = None; l.take_left_most() },
        }
    }
}

impl<T> Iterator for IntoIter_Post<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.remove_left_most();
        return node;
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

impl<V> SearchTree<V> for BST<V>
    where V : Debug + Copy + Clone + Ord + PartialEq {

    fn new(val: V) -> Self {
        BST { val, right : None, left : None, depth : 0 }
    }

    /// https://gist.github.com/aidanhs  Binary Search Tree Tutorial
    fn insert(&mut self, new_val: V) {
        if self.val == new_val {
            return
        }
        let target_node = if self.val > new_val { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = BST::new(new_val);
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    fn remove(&mut self) {
        unimplemented!()
    }

    fn get_val(&self) -> &V {
        &self.val }

    /*
    fn iter(&self) -> &Self {
        unimplemented!()
    }
    */

    fn into_iter_postorder(self) -> IntoIter<V> {
        IntoIter(self)
    }

    /*
    fn iter_mut(&mut self) -> Self {
        unimplemented!()
    }
    */
}

#[test]
fn add_node() {
    let mut new_node : BST<i32> = BST::new(10);
    new_node.insert(7);
    new_node.insert(13);
    assert_eq!(&10, new_node.get_val());
    assert_eq!(&7, new_node.left.unwrap().get_val());
    assert_eq!(&13, new_node.right.unwrap().get_val());
}

