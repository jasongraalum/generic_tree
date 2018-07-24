// Copyright (c) 2018 Jason Graalum //
// Crate which defines a hierarchical tree of generic objects
//
use std::fmt::{Debug,Formatter};
use std::fmt;
use std::prelude::v1::Clone;
use std::prelude::v1::Vec;

/// An TreeNode element contains the data. It needs to include the Clone traits
///  Design decision needed - do we include a path back up the tree??
/// Comment out parent for now to get past reference lifetime issues!
pub struct TreeNode<V>
    where V: Ord + PartialEq + Clone + Copy + Debug {
    val: V,
    children: Vec<TreeNode<V>>,
    //parent: &'a TreeNode<V>,
}

impl<V> PartialEq for TreeNode<V>
    where V: Ord + PartialEq + Clone + Copy + Debug {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<V> Clone for TreeNode<V>
    where V: Ord + PartialEq + Clone + Copy + Debug {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl<V> Debug for TreeNode<V>
    where V: Ord + PartialEq + Clone + Copy + Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

#[allow(dead_code)]
impl<V> TreeNode<V>
    where V: Ord + PartialEq + Clone + Copy + Debug {

    /// Create a new ObjTree with a value and key
    fn new(obj : V) -> Self
    {
        TreeNode { val:  obj, children: Vec::new() }
    }

    /// Add a sub tree to the current tree at the current level
    /// The sub tree is consumed by the add function
    fn add(&mut self, subtree: Self) {
        self.children.push(subtree);
    }

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn split(&mut self, key: TreeNode<V>) -> Result<Self,()> {
        unimplemented!()
    }

    /// &Return the map of references to the subtrees of the current tree
    fn children(&self) ->  &Vec<TreeNode<V>> {
        return &(self.children);
    }

    /// Return a a vector of references to the sibling trees of the current tree
    fn siblings(&self) ->  Vec<&Self> {
        unimplemented!()
    }

    /// Return the degree of the current tree - number of children
    fn degree(&self) -> usize {
        unimplemented!()
    }

    /*
    Add back in once parent is included in TreeNode
    /// Return the height of the current tree
    fn height(&self) -> usize {
        match &self.parent {
            &None => 1,
            &Some(ref p) => {
                1 + p.height().clone()
            }
        }
    }
    */

    /// Return the height of the current tree
    fn depth(&self) -> usize {
        unimplemented!()
    }

    fn get_val(&self) -> &V {
        return &self.val;
    }


}

#[test]
fn add_tree() {
    let mut new_node1 : TreeNode<u32> =  TreeNode::new(10);
    let mut new_node2 : TreeNode<u32> =  TreeNode::new(20);
    let mut new_node3 : TreeNode<u32> =  TreeNode::new(30);
    new_node1.push(new_node2);
    println!("{:?}", new_node1);

    assert_eq!("obj: 20 obj_hash: ObjHash(17869338426324682920) tree_hash: ObjTreeHash(0)", format!("{:?}", new_node1));
}
