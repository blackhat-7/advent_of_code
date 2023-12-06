use std::rc::Rc;

#[derive(Clone, Debug)]
struct Lol {
    num: i32
}

fn main() {
    let mut x: Vec<Rc<Lol>> = Vec::new();
    let y = Lol { num: 1 };
    let z = Rc::new(y);
    x.push(z);
    dbg!(x);
}
