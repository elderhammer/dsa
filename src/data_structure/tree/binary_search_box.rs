use std::cmp::Ord;
use std::ptr::NonNull;
use std::cmp::Ordering;

pub struct BinarySearchTree<T: Ord> {
    root: Tree<T>,
    size: usize
}

pub struct Node<T: Ord> {
    data: T,
    left: Tree<T>,
    right: Tree<T>
}

// 继续用 Option 的好处是不用额外实现它那些好用的方法
pub struct Tree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: Tree(None), // 这种设计太妙了，表示了 nullptr，真的很方便操作，省了提前判断
                              // 因为节点可达，并且直接设值即可，还把 add_left、add_right 这些方法给省了
            right: Tree(None)
        }
    }
}

impl<T: Ord> Tree<T> {
    // 创建一棵空树
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn min(&self) -> Option<&T> {
        let mut current = self;
        let mut parent = current;
        
        while let Some(ref node) = current.0 {
            parent = current;
            current = &node.left;
        }

        parent.0.as_ref().map(|ref node| &node.data)
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = self;
        let mut parent = current;

        while let Some(ref node) = current.0 {
            parent = current;
            current = &node.right;
        }

        parent.0.as_ref().map(|ref node| &node.data)
    }

    // true 表示成功插入，false 表示重复了
    pub fn insert(&mut self, new_data: T) -> bool {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.data.cmp(&new_data) {
                Ordering::Less => current = &mut node.right,
                Ordering::Greater => current = &mut node.left,
                Ordering::Equal => return false
            }
        }

        current.0 = Some(Box::new(Node::new(new_data)));

        true
    }

    pub fn remove(&mut self, target: &T) -> bool {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // init a numric tree
    //          6
    //        2   9
    //      1   3
    //    0
    fn a_numric_tree() -> Tree<i32> {
        let mut tree: Tree<i32> = Tree::empty();
        assert!(tree.insert(6));
        assert!(tree.insert(2));
        assert!(tree.insert(1));
        assert!(tree.insert(0));
        assert!(tree.insert(9));
        assert!(tree.insert(3));
        tree   
    }

    #[test]
    fn bsb_insert() {
        let mut tree = a_numric_tree();
        assert!(tree.insert(1) == false);
    }

    #[test]
    fn bsb_min() {
        let mut tree = a_numric_tree();
        assert_eq!(tree.min(), Some(&0i32));
    }

    #[test]
    fn bsb_max() {
        let mut tree = a_numric_tree();
        assert_eq!(tree.max(), Some(&9i32));
    }

    // #[test]
    // fn bsb_remove() {
    //     let mut tree = a_numric_tree();
    //     tree.remove(&2i32);
    //     assert_eq!(tree.max(), Some(&9i32));
    //     assert_eq!(tree.min(), Some(&0i32));
    // }
}