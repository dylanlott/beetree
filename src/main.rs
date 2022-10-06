// Node defines a generic type for nodes in our binary tree.
// We need PartialEq to let our tree work on comparisons for i32 apparently.
#[derive(PartialEq)]
pub struct Node<T>{
    val: T, // what does &'a mean? It's called a "shared reference" and the 'a part means specifically "the lifetime of a"
    left: ChildNode<T>,
    right: ChildNode<T> 
}

// ChildNode makes it a little cleaner to deal with Node and makes it explicit how we intend to use that type.
pub type ChildNode<T> = Option<Box<Node<T>>>;

// BinaryTree defines our tree struct that will be a collection of Some Node types.
// pub struct BinaryTree<T> {
//     head: Option<Node<T>>,
// }

// Node of i32 type for our first implementation.
impl<'a > Node<i32> {
    pub fn insert(&mut self, new_val: i32) {
        if self.val == new_val {
            return
        }

        // detect if we have left or right children 
        let target_node= if new_val < self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        // match on the case of the target node
        match target_node {
            // if that node exists, recursively call insert on it
            // subnode needs to be ref mut because it's behind a pointer
            Some(ref mut subnode) => subnode.insert(new_val),
            // if it doesn't exist, then we assign it to what we expect it to be
            None => {
                let new_node = Node { val: new_val, left: None, right: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node
            }
        }
    }
}

fn main() {
    println!("Hello, beetree!");
    let mut node = Node{val: 0, left: None, right: None};
    node.insert(2);
    node.insert(1);
    assert!(node == Node {
        val: 0,
        left: None,
        right: Some(Box::new(Node {
            val: 2,
            left: Some(Box::new(Node {
                val: 1,
                left: None,
                right: None,
            })),
            right: None
        }))
    })
}
