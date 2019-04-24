use std::ops::Deref;

#[derive(Debug)]
struct Test(i32);

impl Deref for Test {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

fn main() {
    let t = Test(12);

    let p = *t;

    println!("{:#?}", t);
}