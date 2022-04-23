enum Return {
    Int(usize),
    String(String),
    Bool(bool),
}

fn do_something(num: usize) -> Return {
    match num {
        1 => Return::Int(1),
        2 => Return::String(String::from("two")),
        3 => Return::Bool(true),
        _ => unreachable!(),
    }
}

pub fn main(){

    if let Return::Int(i) = do_something(1) {
        println!("{}",i); // print 1
    }
    if let Return::String(s) = do_something(2) {
        println!("{}",s); // prints two
    }
    if let Return::Bool(b) = do_something(3) {
        println!("{}",b); // prints true
    }
}