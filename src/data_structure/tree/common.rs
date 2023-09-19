use std::ptr::NonNull;
use std::marker::PhantomData;
use std::collections::LinkedList;
use std::fmt::Display;

pub struct Common<T: Display> {
    root: Link<T>,
    _phantom: PhantomData<T>
}

pub type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Display> {
    elem: T,
    //parent: Link<T>,
    first_child: Link<T>,
    next_siblings: Option<LinkedList<Link<T>>>
}

#[derive(Debug)]
pub enum FileCat {
    Dir,
    File
}

impl Display for FileCat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Dir => "dir",
            Self::File => "file"
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct FileData {
    name: String, // 文件名
    cat: FileCat, // 文件类型
    size: u32 // 文件大小，单位是字节
}

impl FileData {
    pub fn new_file(name: &str, size: u32) -> Self {
        Self::new(name, FileCat::File, size)
    }

    pub fn new_dir(name: &str) -> Self {
        Self::new(name, FileCat::Dir, 1)
    }

    fn new(name: &str, cat: FileCat, size: u32) -> Self {
        Self {
            name: name.to_string(),
            cat,
            size
        }
    }

    pub fn to_node(self) -> Link<FileData> {
        Node::new(self).to_link()
    }

    pub fn print_name(&self) {
        println!("{}", self.name)
    }

    pub fn print_size(&self) {
        println!("{}", self.size)
    }
}

impl Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.size)
    }
}

impl<T: Display> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem,
            first_child: None,
            next_siblings: None
        }
    }

    // 插入新的长子，返回旧的长子
    pub fn add_first_child(&mut self, elem: T) -> Link<T> {
        let old_child = self.first_child.take();

        self.first_child = Node::new(elem).to_link();

        old_child
    }

    pub fn add_next_siblings(&mut self, elem: T) {
        let next_sibling = Node::new(elem).to_link();

        if self.next_siblings.is_none() {
            let mut next_siblings = LinkedList::new();
            self.next_siblings = Some(next_siblings);
        }
        self.next_siblings.as_mut().unwrap().push_back(next_sibling);
    }

    pub fn preorder_traversal(&self, depth: i32) {
        // 缩进
        let mut tab = "".to_string();
        for i in 0..(depth * 4) {
            tab.push_str(" ");
        }

        // 根
        println!("{}{}", tab, &self.elem);

        // 左
        self.first_child.as_ref().map(|node| {
            unsafe {
                node.as_ref().preorder_traversal(depth+1)
            }
        });

        // 右
        self.next_siblings.as_ref().map(|next_siblings| {
            for option_node in next_siblings.iter() {
                option_node.as_ref().map(|node| {
                    unsafe {
                        node.as_ref().preorder_traversal(depth+1)
                    }
                });
            }
        });
    }

    pub fn to_link(self) -> Link<T> {
        NonNull::new(Box::into_raw(Box::new(self)))
    }
}

impl<T: Display> Common<T> {
    pub fn new(node: Link<T>) -> Self {
        Self {
            root: node,
            _phantom: PhantomData
        }
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root_dir() -> FileData {
        FileData::new_dir("/usr")
    }

    #[test]
    fn new() {
        
    }

    #[test]
    fn add_first_child() {

    }

    #[test]
    fn add_next_siblings() {

    }

    #[test]
    fn preorder_traversal() {

    }
}