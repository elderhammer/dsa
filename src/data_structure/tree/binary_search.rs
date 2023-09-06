use std::ptr::NonNull;
use std::fmt::{Display, write};
use std::cmp::Ord;

pub type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Display + Ord> {
    elem: T,
    left_child: Link<T>,
    right_child: Link<T>
}

impl<T: Display + Ord> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            left_child: None,
            right_child: None
        }
    }

    pub fn add_left_child(&mut self, node: Node<T>) -> Link<T> {
        let old_child = self.left_child.take();

        self.left_child = node.to_link();

        old_child
    }

    pub fn add_right_child(&mut self, node: Node<T>) -> Link<T> {
        let old_child = self.right_child.take();

        self.right_child = node.to_link();

        old_child
    }

    // 左 根 右
    pub fn inorder_traversal(&self) {
        if self.left_child.is_some() {
            print!("( ");
            unsafe {
                self.left_child.as_ref().unwrap().as_ref().inorder_traversal();
            }
        }

        print!(" {} ", self.elem);

        if self.right_child.is_some() {
            unsafe {
                self.right_child.as_ref().unwrap().as_ref().inorder_traversal();
            }
            print!(" )");
        }
    }

    // 根 左 右
    pub fn preorder_traversal(&self) {
        print!(" {} ", self.elem);

        if self.left_child.is_some() {
            unsafe {
                self.left_child.as_ref().unwrap().as_ref().preorder_traversal();
            }
        }

        if self.right_child.is_some() {
            unsafe {
                self.right_child.as_ref().unwrap().as_ref().preorder_traversal();
            }
        }
    }

    pub fn contains(&self, target: &T) -> bool {
        if &self.elem < target {
            if self.right_child.is_some() {
                unsafe {
                    self.right_child.as_ref().unwrap().as_ref().contains(target)
                }
            } else {
                false
            }
        } else if target < &self.elem {
            if self.left_child.is_some() {
                unsafe {
                    self.left_child.as_ref().unwrap().as_ref().contains(target)
                }
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn insert(&mut self, target: T) {
        let new_node = Node::new(target);

        let node = self.locate_mut(&new_node.elem);
        if new_node.elem < node.elem {
            node.add_left_child(new_node);
        } else if node.elem < new_node.elem {
            node.add_right_child(new_node);
        }
    }

    pub fn remove(link: &mut Link<T>, target: &T) -> Link<T> {
        unsafe {
            let node = link.as_mut().unwrap().as_mut();
            if target < &node.elem {
                if node.left_child.is_some() {
                    Self::remove(&mut node.left_child, target)
                } else {
                    None
                }
            } else if &node.elem > target {
                if node.right_child.is_some() {
                    Self::remove(&mut node.right_child, target)
                } else {
                    None
                }
            } else if &node.elem == target {
                if node.left_child.is_none() && node.right_child.is_none() {
                    link.take()
                } else if node.left_child.is_some() && node.right_child.is_none() {
                    let mut removed_link = link.take();
                    link.replace(removed_link.as_mut().unwrap().as_mut().left_child.take().unwrap())
                } else if node.left_child.is_none() && node.right_child.is_some() {
                    let mut removed_link = link.take();
                    link.replace(removed_link.as_mut().unwrap().as_mut().right_child.take().unwrap())
                } else {
                    link.take()
                }
            } else {
                None
            }
        }
    }

    pub fn locate_mut(&mut self, target: &T) -> &mut Node<T> {
        let mut node = self;
        'outer: loop {
            if &node.elem < target {
                if node.right_child.is_some() {
                    unsafe {
                        node = node.right_child.as_mut().unwrap().as_mut()
                    }
                } else {
                    break 'outer;
                }
            } else if  target < &node.elem {
                if node.left_child.is_some() {
                    unsafe {
                        node = node.left_child.as_mut().unwrap().as_mut()
                    }
                } else {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }

        node
    }

    pub fn to_link(self) -> Link<T> {
        NonNull::new(Box::into_raw(Box::new(self)))
    }
}

impl<T: Display + Ord> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elem)
    }
}

pub struct BSTree<T: Display + Ord> {
    root: Link<T>
}

impl<T: Display + Ord> BSTree<T> {
    pub fn new(root: Link<T>) -> Self {
        Self { root }
    }

    pub fn contains(&self, target: &T) -> bool {
        if self.root.is_none() {
            false
        } else {
            unsafe {
                self.root.as_ref().unwrap().as_ref().contains(target)
            }
        }
    }

    pub fn insert(&mut self, target: T) {
        if self.root.is_some() {
            unsafe {
                self.root.as_mut().unwrap().as_mut().insert(target);
            }
        } else {
            self.root = Node::new(target).to_link();
        }
    }

    pub fn preorder_traversal(&self) {
        if let Some(node) = &self.root {
            unsafe {
                node.as_ref().preorder_traversal()
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_numric_tree() -> BSTree<i32> {
        let three = Node::new(3);

        let mut four = Node::new(4);
        four.add_left_child(three);

        let mut two = Node::new(2);
        two.add_left_child(Node::new(1));
        two.add_right_child(four);

        let mut six = Node::new(6);
        six.add_left_child(two);

        let eight = Node::new(8);
        six.add_right_child(eight);

        BSTree::new(six.to_link())
    }

    #[test]
    fn contains() {
        let tree = a_numric_tree();

        let target_two = 2;
        assert!(tree.contains(&target_two));

        let target_seven = 7;
        assert!(tree.contains(&target_seven) == false);

        let target_six = 6;
        assert!(tree.contains(&target_six));
    }

    #[test]
    fn insert() {
        let mut tree = a_numric_tree();

        let target: i32 = 7;

        assert!(tree.contains(&target) == false);

        tree.insert(target);
        assert!(tree.contains(&target));

        let mut tree: BSTree<i32> = BSTree::new(None);
        let numbers = [6, 2, 1, 4, 3, 8, 7];
        for number in numbers {
            tree.insert(number);
        }

        tree.preorder_traversal();
    }
}