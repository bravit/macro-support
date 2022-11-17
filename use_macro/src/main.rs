use my_macro::{make_answer, declare_variables};

make_answer!(1 + 2);

fn main() {
    println!("{}", answer());

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