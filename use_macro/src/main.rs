use my_macro::{NameFn, declare_variables};

trait Name {
    fn name() -> String;
}

#[derive(NameFn)]
struct Info;

fn main() {
    println!("Named item: {}", Info::name());

    declare_variables! {
        a = 'z',
        b = 2,
        c = a,
        d,
        e = "some string"
    }
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
}