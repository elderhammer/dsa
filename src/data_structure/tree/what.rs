#[derive(Debug)]
pub struct Foo<T> {
    bar: T
}

impl<T> Foo<T> {
    pub fn new(bar: T) -> Self {
        Self { bar }
    }

    pub fn peek_mut(&mut self) -> &mut T {
        &mut self.bar
    }

    pub fn self_mut(&mut self) -> &mut Self {
        &mut *self
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn foo() {
        let mut foo = Foo::new(1);
        let foo_ref = &mut foo;
        let bar_ref = foo_ref.peek_mut();
        //foo_ref.bar = 3;
        *bar_ref = 2;
        //foo_ref.bar = 3;
        print!("{:?}", foo_ref);
    }

    #[test]
    fn bar() {
        let mut foo = Foo::new(1);
        let foo_ref = &mut foo;
        let bar_ref = foo_ref.self_mut();
        bar_ref.bar = 2;
        foo_ref.bar = 3;
        println!("{:?}", foo_ref);
    }
}