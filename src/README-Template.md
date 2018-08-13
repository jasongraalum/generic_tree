# Generic Tree in Rust

Our original goal was to create a more general framework that could be used for multiple types of trees. This turned out to be too large of a task for the scope of our project since we could not figure how to accomplish this without implementing a separate generic tree for each tree type.
Our first, (and most likely final) version is an implementation for a generic binary search tree.  Generic tree currently implements rust standard iterators, iter() and into_iter() for pre_order, in_order and post_order traversal. Other methods include: 
- iter()
- iter_in_order()
- iter_pre_order()
- iter_post_order()
- insert(val: V):                   Inserts BSTNode into BST
- remove(val: V) -> option<V>:      Removes Node by data value
- min_depth() -> u64:               Gets depth of the last full row
- height() -> u64:                  Gets height of tree.
- size() -> usize
- find(val: V) -> Option(V):        Returns None if value is not found.
- contains(val: V) ->               Boolean true or false. 
- min_value() 
- swap_right():                     Swap values of the current BST with the right node BST. Return the current BST.
- swap_left():                      Swap values of the current BST with the left node BST. Return the current BST.
- take_right():                     Removes right subtree.
- take_left():                      Removes left subtree.
- merge(other_tree: BST<V>):        Merges current tree with another tree by inserting values from other.
- peek() -> Option<&V>:             Gets roots data value


## Getting Started
- Go to https://github.com/jasongraalum/generic_tree.
- Copy and paste the contents of src/lib.rs into your rust project.
- Try it out.

### Prerequisites
- rustc version 1.27.0 or greater
-- https://www.rust-lang.org/en-US/install.html


### Installing

A step by step series of examples that tell you how to get a development env running

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo

## Running the tests

- Navigate to the project directory in your terminal. 
- Enter 'cargo build' on the command line
- Enter 'cargo test' to run the tests

### Break down into end to end tests



```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

rust-lang/cargo - The Rust package manager


## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Jason Graalum and Nathan Reed** - *Initial work* - [PurpleBooth](https://github.com/jasongraalum/generic_tree)

See also the list of [contributors](https://github.com/jasongraalum/generic_tree/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Special thanks to Learning Rust With Entirely Too Many Linked Lists: http://cglab.ca/~abeinges/blah/too-many-lists/book/ for helping us understand working with iterators a little better.
c

