// Copyright (c) 2018 Jason Graalum //
// Crate which defines a hierarchical tree of generic objects
//
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;

trait GenericTree<K, V, N = Self>
    where K : Ord, V : Hash {
    /// Return a newly created Tree
    fn new(key : K, val : V) -> Self;

    /// Add a sub tree to the current tree at the current level
    fn add(&mut self, subtree: Self);

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn rm<'a>(&mut self, key: K) -> Result<N,()>;

    /// Return a references to the subtree hashmap of the current tree
    fn children(&self) ->  &HashMap<N,K>;

    /// Return a references to the parent tree of the current tree
    fn parent(&self) -> Option<&Self>;

    /// Return a a vector of references to the sibling trees of the current tree
    fn siblings(&self) ->  Vec<&Self>;

    /// Return the degree of the current tree
    fn degree(&self) -> usize;

    /// Return the height of the current tree
    fn height(&self) -> usize;

    /// Return the height of the current tree
    fn depth(&self) -> usize;

    /// Return an unique index/hash value for the root of the current tree
    fn get_key(&self) -> K;

    /// Return the value of the current tree root
    fn get_val(&self) -> V;
}

/// An ObjTree element contains a optional reference to it's parent and a HashMap of it's children
/// T represents the type of data contained in the ObjTree
/// K represents the type of the HashMap key i.e. u8, u32 or [u8;32]
///
pub struct ObjTree<K, V>
    where K: Ord, V: Hash {
    key: K,
    val: V,
    children: HashMap<ObjTree<K, V>, K>,
    parent: Box<ObjTree<K, V>>,
}

impl<K: Ord + Eq, V: Hash > PartialEq for ObjTree<K, V> {
    fn eq(&self, other: &ObjTree<K, V>) -> bool {
        self.key == other.key
    }
}

impl<K: Ord + Eq, V: Hash> Eq for ObjTree<K, V> {}

impl<K: Ord + Eq, V: Hash > Hash for ObjTree<K, V> {
   fn hash<H: Hasher>(&self, state: &mut H) {
       self.val.hash(state);
   }
}

impl<'a, K: Debug + Ord + Eq, V: Debug + Hash> GenericTree<K, V> for ObjTree<K, V>
    where K: Ord, V: Hash {
    /// Create a new, empty ObjTree
    fn new(key : K, val : V) -> Self
    {
       ObjTree { key : key, val : val, children: HashMap::new(), parent : Option<Box<ObjTree<K, V>> }
    }

    /// Add a sub tree to the current tree at the current level
    /// The sub tree is consumed by the add function
    fn add(&mut self, subtree: Self) {
        self.children.insert(subtree, subtree.get_key());
    }

    /// Remove a subtree from the current tree and return ownership of the removed subtree.
    fn rm(&mut self, key: K) -> Result<Self,()> {
        ()
    }

    /// &Return the map of references to the subtrees of the current tree
    fn children(&self) ->  &HashMap<ObjTree<K, V>,K> {
        &(self.children)
    }

    /// Return a references to the parent tree of the current tree
    fn parent(&self) -> Option<&Self> {
        &self.parent
    }

    /// Return a a vector of references to the sibling trees of the current tree
    fn siblings(&self) ->  Vec<&Self> {
        &self.parent.children()
    }

    /// Return the degree of the current tree - number of children
    fn degree(&self) -> usize {
        (&self.children).len()
    }

    /// Return the height of the current tree
    fn height(&self) -> usize {
        match &self.parent {
            None => 1,
            p => 1 + p.height()
        }
    }

    /// Return the height of the current tree
    fn depth(&self) -> usize {
        0
    }

    fn get_key(&self) -> K {
        return self.key;
    }

    fn get_val(&self) -> V {
        return self.val;
    }
}
