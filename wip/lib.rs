// Copyright (c) 2018 Jason Graalum //
// Crate which defines a hierarchical tree of generic objects
//
use std::fmt::{Debug,Formatter};
use std::fmt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use std::prelude::v1::Clone;
use std::prelude::v1::Vec;

/// Wrap u64 as ObjHash and ObjTreeHash types for future use
#[derive(Debug,Clone,Hash)]
pub struct ObjHash(u64);

#[derive(Debug,Clone,Hash)]
pub struct ObjTreeHash(u64);

impl<V> Debug for ObjNode<V>
    where V: Hash + Clone + Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.val)
    }
}

/// An ObjTree element contains a optional reference to it's parent and a Vector of it's children
/// T represents the type of data contained in the ObjTree
/// K represents the type of the hash key i.e. u8, u32 or [u8;32]
/// The children vector holds tuples of (ObjTree, ObjTreeHash)
///
pub struct ObjTree<V>
    where V: Debug + Clone + Hash {
    obj: V,
    obj_hash: ObjHash,
    tree_hash: ObjTreeHash,
    children: Vec<ObjTree<V>>,
    parent: Option<Box<ObjTree<V>>>,
}

impl<V> Hash for ObjTree<V>
    where V: Hash + Clone + Debug {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unimplemented!()
    }
}

// Very expensive
impl<V> Clone for ObjTree<V>
    where V: Hash + Clone + Debug {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl<V> Debug for ObjTree<V>
    where V: Hash + Clone + Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "obj: {:?} obj_hash: {:?} tree_hash: {:?}", self.obj, self.obj_hash, self.tree_hash);
        for ref child in &self.children {
            child.fmt(f);
        }
        write!(f,"")
    }
}

#[allow(dead_code)]
impl<'a, V> ObjTree<ObjNode<V>>
    where V: Hash + Clone + Debug {


    /// Create a new ObjTree with a value and key
    fn new(obj : ObjNode<V>) -> Self
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        let obj_hash = ObjHash(hasher.finish());

        //let tree_hash = calculate_tree_hash(&obj_hash);
        let tree_hash = ObjTreeHash(0);
        ObjTree { obj, obj_hash, tree_hash, children: Vec::new(), parent : None }
    }

    /// Add a sub tree to the current tree at the current level
    /// The sub tree is consumed by the add function
    fn add(&mut self, subtree: Self) {
        self.children.push(subtree);
        self.tree_hash = self.calculate_tree_hash(&self.obj_hash);
    }

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn split(&mut self, key: ObjNode<V>) -> Result<Self,()> {
        unimplemented!()
    }

    /// &Return the map of references to the subtrees of the current tree
    fn children(&self) ->  &Vec<ObjTree<ObjNode<V>>> {
        return &(self.children);
    }

    /// Return a references to the parent tree of the current tree
    fn parent(&self) -> Option<&Self> {
        unimplemented!()
    }

    /// Return a a vector of references to the sibling trees of the current tree
    fn siblings(&self) ->  Vec<&Self> {
        unimplemented!()
    }

    /// Return the degree of the current tree - number of children
    fn degree(&self) -> usize {
        unimplemented!()
    }

    /// Return the height of the current tree
    fn height(&self) -> usize {
        match &self.parent {
            &None => 1,
            &Some(ref p) => {
                1 + p.height().clone()
            }
        }
    }

    /// Return the height of the current tree
    fn depth(&self) -> usize {
        unimplemented!()
    }

    fn get_tree_hash(&self) -> &ObjTreeHash {
        return &self.tree_hash;
    }

    fn get_obj_hash(&self) -> &ObjHash {
        return &self.obj_hash;
    }

    fn get_val(&self) -> &ObjNode<V> {
        return &self.obj;
    }

    fn calculate_tree_hash(&self, obj_hash: &ObjHash) -> ObjTreeHash {
        let mut s = DefaultHasher::new();
        for ref child in &self.children {
            let val = child.obj_hash.0.clone();
            s.write_u64(val);
        }
        ObjTreeHash(s.finish())
    }

}

#[test]
fn add_tree() {
    let new_node1 =  ObjNode {val : 20};
    let new_node2 =  ObjNode {val : 30};
    let new_node3 =  ObjNode {val : 40};
    let mut tree1: ObjTree<ObjNode<i32>> =  ObjTree::new(new_node1);
    let mut tree2 : ObjTree<ObjNode<i32>> =  ObjTree::new(new_node2);
    let tree3 : ObjTree<ObjNode<i32>> =  ObjTree::new(new_node3);
    tree2.add(tree3);
    tree1.add(tree2);
    println!("{:?}", tree1);

    assert_eq!("obj: 20 obj_hash: ObjHash(17869338426324682920) tree_hash: ObjTreeHash(0)", format!("{:?}", tree1));
}
