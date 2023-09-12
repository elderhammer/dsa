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

    pub fn find_min(&self) -> Option<&T> {
        self.0.map(|ref node| {
            
        })
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
            current.0 = NonNull::new(Box::into_raw(Box::new(Node::new(new_data))));
        }

        true
    }

    // true 表示成功删除，false 表示目标不存在
    pub fn remove(&mut self, target: &T) -> bool {
        let mut current = self;

        unsafe {
            while let Some(ref mut node) = current.0 {
                match node.as_ref().data.cmp(&target) {
                    Ordering::Less => current = &mut node.as_mut().right,
                    Ordering::Greater => current = &mut node.as_mut().left,
                    Ordering::Equal => match (node.as_ref().left.0, node.as_ref().right.0) {
                        (None, None) => {
                            // 没有孩子
                            current.0.take();
                            return true;
                        }
                        (Some(_), None) => {
                            // 只有左孩子
                            current.0 = node.as_mut().left.0.take();
                            return true;
                        }
                        (None, Some(_)) => {
                            // 只有右孩子
                            current.0 = node.as_mut().right.0.take();
                            return true;
                        }
                        (Some(_), Some(_)) => {
                            // 有两个孩子
                            let right = node.as_mut().right.0.take();

                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst2_insert() {
        let mut tree: Tree<i32> = Tree::empty();
        assert!(tree.insert(1));
        assert!(tree.insert(2));
        assert!(tree.insert(1) == false);
    }
}