use std::ptr::NonNull;
use std::fmt::{Display, write};

pub type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T: Display> {
    elem: T,
    left_child: Link<T>,
    right_child: Link<T>
}

impl<T: Display> Node<T> {
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

        print!(" {} ", self);

        if self.right_child.is_some() {
            unsafe {
                self.right_child.as_ref().unwrap().as_ref().inorder_traversal();
            }
            print!(" )");
        }
    }

    // 根 左 右
    pub fn preorder_traversal(&self) {
        print!(" {} ", self);
        if self.left_child.is_some() {
            //print!("( ");
            unsafe {
                self.left_child.as_ref().unwrap().as_ref().preorder_traversal();
            }
        }

        if self.right_child.is_some() {
            unsafe {
                self.right_child.as_ref().unwrap().as_ref().preorder_traversal();
            }
            //print!(" )");
        }
    }

    pub fn to_link(self) -> Link<T> {
        NonNull::new(Box::into_raw(Box::new(self)))
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elem)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_expression_tree() {
        // input
        let postfix = "ab+cde+**";

        // stack
        let mut stack: Vec<Node<char>> = Vec::new();

        for char in postfix.chars() {
            if char.is_ascii_lowercase() {
                // operand
                // 创建一个单节点树，入栈
                let new_operand = Node::new(char);
                stack.push(new_operand);
            } else {
                // operator
                // 出栈两次，分别作为左右孩子，然后入栈
                let mut new_operator = Node::new(char);
                let right_child = stack.pop().unwrap();
                let left_child = stack.pop().unwrap();
                new_operator.add_left_child(left_child);
                new_operator.add_right_child(right_child);
                stack.push(new_operator);
            }
        }

        // 出栈
        let expression_tree = stack.pop().unwrap();

        // 遍历打印
        expression_tree.inorder_traversal();
        println!("");

        expression_tree.preorder_traversal();
        println!("");
    }
}