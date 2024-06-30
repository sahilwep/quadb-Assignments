/*
//  Question: Given a binary tree, implement a function that returns the maximum depth of the tree.

// Observations: 
    * We need to return the depth of binary tree..
    * We can use recursive solution for this, we will start from root node, & until we can't reach to last node or null, we recursively call for the right sub-tree & left sub-tree & take maximum of them & add 1(because of each level) & return...

// Examples: 
    * Maximum number of node from root to leaf path.
    I/p : 
                                            [10]
                                              |
                                      [8]-----------[30]
                                                       |
                                                [40]---------[50]
                                                              |
                                                        [70]------[90]
    O/p : 4


    I/p :               
                                            [10]
    O/p : 1


    I/p : NULL
    O/p : 0

    I/p :       
                                            [30]
                                              |
                                      [40]----------[60]
                                        |
                                [70]--------[80]
                                  |
                         [80]----------[90]
    O/p : 4


    I/p :       
                                            [10]
                                              |
                                         [12]-------[20]
                                                     |
                                                [40]-----[20]

    O/p : 3

// Intrusion: 

    * We will recursively got to right subtree & then left subtree, & we wil compute depth..

                        [10]
                          |
                  [8]-----------[30]
                                   |
                            [40]---------[50]

    * We can use max function to get the depth of recursive tree..

    // Recursive tree:
                depth(10) : gets max(1, 2) + 1 -> 3, & return to the parent call.
                    |--->depth(8) : gets 0, & add 1 to it & then return to parent call.
                    |       |--->depth(NULL) : return 0
                    |       |--->depth(NULL) : return 0
                    |
                    |--->depth(30) : gets max(1, 1) + 1 -> have 2, & return 2 to the parent call
                            |--->depth(40) : gets 0, & add 1 to it & then return to parent call.
                            |       |--->depth(NULL)
                            |       |--->depth(NULL)
                            |
                            |--->depth(50) : gets 0, & add 1 to it & then return to parent call.
                                   |--->depth(NULL)
                                   |--->depth(NULL)

        * It will return 3, as depth..

// Time Complexity: 
    O(n), for n nodes
        * for every node it's take O(1) constant work, as we are doing comparision & addition..

*/

use std::rc::Rc;
use std::cell::RefCell;

// Define the Node struct
struct Node {
    key: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            left: None,
            right: None,
        }))
    }
}

// Function to calculate the depth of the binary tree
fn depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            1 + std::cmp::max(depth(node.left.clone()), depth(node.right.clone()))
        }
        None => 0,
    }
}

// Function for pre-order traversal
fn pre_order(root: Option<Rc<RefCell<Node>>>) {
    if let Some(node) = root {
        let node = node.borrow();
        print!("{} ", node.key);
        pre_order(node.left.clone());
        pre_order(node.right.clone());
    }
}

fn main() {
    let root = Node::new(10);
    {
        let mut root_borrow = root.borrow_mut();
        root_borrow.left = Some(Node::new(8));
        root_borrow.right = Some(Node::new(30));
    }
    {
        let right = root.borrow().right.clone().unwrap();
        let mut right_borrow = right.borrow_mut();
        right_borrow.left = Some(Node::new(40));
        right_borrow.right = Some(Node::new(50));
    }

    pre_order(Some(root.clone()));
    println!();
    println!("Depth: {}", depth(Some(root.clone())));
}
