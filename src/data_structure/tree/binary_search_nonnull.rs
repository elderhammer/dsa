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
pub struct Tree<T: Ord>(Option<NonNull<Node<T>>>);

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
        unsafe {
            while let Some(ref node) = current.0 {
                parent = current;
                current = &node.as_ref().left;
            }

            parent.0.map(|ref node| &node.as_ref().data)
        }
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = self;
        let mut parent = current;
        unsafe {
            while let Some(ref node) = current.0 {
                parent = current;
                current = &node.as_ref().right;
            }

            parent.0.map(|ref node| &node.as_ref().data)
        }
    }

    // true 表示成功插入，false 表示重复了
    pub fn insert(&mut self, new_data: T) -> bool {
        // 如果担心递归导致 T 一直传递 T，那么改成迭代就可以
        let mut current = self;

        unsafe {
            while let Some(ref mut node) = current.0 {
                match node.as_ref().data.cmp(&new_data) {
                    Ordering::Less => current = &mut node.as_mut().right,
                    Ordering::Greater => current = &mut node.as_mut().left,
                    Ordering::Equal => return false
                }
            }

            // 插入新值
            current.0 = Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(new_data)))));
        }

        true
    }

    pub fn remove(&mut self, target: &T) -> Option<Tree<T>> {
        let mut current = self;

        unsafe {
            while let Some(ref mut node) = current.0 {
                match node.as_ref().data.cmp(&target) {
                    // 在 safe 世界里，把后继引用赋值给前驱引用是非法的，因为被引用的情况下不能被修改
                    // 但在 unsafe 世界里，这只是改个指针
                    Ordering::Less => current = &mut node.as_mut().right,
                    Ordering::Greater => current = &mut node.as_mut().left,
                    Ordering::Equal => match (node.as_ref().left.0, node.as_ref().right.0) {
                        (None, None) => {
                            // no child
                            return Some(Tree(current.0.take()));
                        }
                        (Some(_), None) => {
                            // only left
                            let left = node.as_mut().left.0.take();
                            let removed_tree = Some(Tree(current.0.take()));
                            current.0 = left;
                            return removed_tree;
                        }
                        (None, Some(_)) => {
                            // only right
                            let right = node.as_mut().right.0.take();
                            let removed_tree = Some(Tree(current.0.take()));
                            current.0 = right;
                            return removed_tree;
                        }
                        (Some(_), Some(_)) => {
                            // two childs
                            let min_of_right = node.as_ref().right.min().unwrap();
                            // 候选节点
                            let mut candidate_tree = node.as_mut().right.remove(&min_of_right).unwrap();
                            let mut candidate_node = candidate_tree.0.as_mut().unwrap().as_mut();
                            // 当前节点
                            let mut removed_tree = Tree(current.0.take());
                            let removed_node = removed_tree.0.as_mut().unwrap().as_mut();
                            // 继承孩子
                            candidate_node.left.0 = removed_node.left.0.take();
                            candidate_node.right.0 = removed_node.right.0.take();
                            // 即位
                            *current = candidate_tree;

                            return Some(removed_tree);
                        }
                    }
                }
            }
        }

        None
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
    fn bst2_insert() {
        let mut tree = a_numric_tree();
        //assert!(tree.insert(1) == false);
    }

    #[test]
    fn bst2_min() {
        let mut tree = a_numric_tree();
        assert_eq!(tree.min(), Some(&0i32));
    }

    #[test]
    fn bst2_max() {
        let mut tree = a_numric_tree();
        assert_eq!(tree.max(), Some(&9i32));
    }

    #[test]
    fn bst2_remove() {
        let mut tree = a_numric_tree();
        tree.remove(&2i32);
        assert_eq!(tree.max(), Some(&9i32));
        assert_eq!(tree.min(), Some(&0i32));
    }
}