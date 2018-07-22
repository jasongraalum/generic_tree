// Copyright (c) 2018 Jason Graalum //
// Crate which defines a hierarchical tree of generic objects
//
use std::collections::HashMap;
use std::fmt::{Debug,Formatter};
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher; use std::prelude::v1::Clone;

/// An ObjTree element contains a optional reference to it's parent and a HashMap of it's children
/// T represents the type of data contained in the ObjTree
/// K represents the type of the HashMap key i.e. u8, u32 or [u8;32]
///
pub struct ObjTree<K, V>
    where K: Ord + Eq + Clone, V: Hash + Clone {
    key: K,
    val: V,
    children: HashMap<ObjTree<K, V>, K>,
    parent: Option<Box<ObjTree<K, V>>>,
}

// Very expensive
impl<K, V> Clone for ObjTree<K, V>
    where K: Ord + Eq + Clone, V: Hash + Clone {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

// PartialEq is only for self - not for the sub tree.
impl<K, V> PartialEq for ObjTree<K, V>
    where K: Ord + Eq + Clone, V: Hash + Clone {
    fn eq(&self, other: &ObjTree<K, V>) -> bool {
        self.key == other.key
    }
}

impl<K, V> Eq for ObjTree<K, V>
where K: Ord + Eq + Clone, V: Hash + Clone {
}

impl<K, V> Hash for ObjTree<K, V>
    where K: Ord + Eq + Clone, V: Hash + Clone {
   fn hash<H: Hasher>(&self, state: &mut H) {
       self.val.hash(state);
   }
}

impl<K, V> Debug for ObjTree<K,V>
    where K: Ord + Eq + Clone + Debug, V: Hash + Clone + Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "key: {:?} val: {:?}", self.key, self.val)
        //write!("key: {:?} val: {:?},", self.key.fmt(f), self.val.fmt(f))
    }
}

#[allow(dead_code)]
impl<'a, K , V> ObjTree<K, V>
    where K: Ord + Clone, V: Hash + Clone {

    /// Create a new ObjTree with a value and key
    fn add_key_value_pair(key : K, val : V) -> Self
    {
        ObjTree { key: key, val: val, children: HashMap::new(), parent : None }
    }

    /// Create a new ObjTree with a value and generated key
    fn add_value(val : V) -> Self
    {
        let key: K = ObjTree::hash(val);
        ObjTree { key: key, val: val, children: HashMap::new(), parent : None }
    }

    /// Add a sub tree to the current tree at the current level
    /// The sub tree is consumed by the add function
    fn add(&mut self, subtree: Self) {
        let sub_key = subtree.get_key();
        self.children.insert(subtree, sub_key);
    }

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn split(&mut self, key: K) -> Result<Self,()> {
        unimplemented!()
    }

    /// &Return the map of references to the subtrees of the current tree
    fn children(&self) ->  &HashMap<ObjTree<K, V>,K> {
        &(self.children)
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
        0
    }

    fn get_key(&self) -> K {
        return self.key.clone();
    }

    fn get_val(&self) -> V {
        return self.val.clone();
    }

}

#[test]
fn add_tree() {
    let new_val: i32 = 20;
    let new_key: u32 = 1;
    let tree : ObjTree<u32, i32> =  ObjTree::new(new_key, new_val);

    assert_eq!("key: 1 val: 20", format!("{:?}", tree));
}
