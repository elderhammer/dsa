use std::ptr::NonNull;

#[derive(Debug)]
pub struct Vector<T> {
    //array: Option<Box<[Option<T>]>>,
    //array: Vec<T>, // Vec<T> 对 T 是 covariant 的，所以 Vector 对 T 也是 covariant 的。
    array: NonNull<T>, // NonNull<T> 对 T 是 covariant 的，所以 Vector 对 T 也是 covariant 的。
    size: i32,
    cap: i32
}

impl<T> Vector<T> {}

