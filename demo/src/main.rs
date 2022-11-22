use macro_examples::{NameFn, declare_variables};

trait Name {
    fn name() -> String;
}

#[derive(NameFn)]
struct Info;

#[derive(NameFn)]
enum Bool3 { True, False, Unknown }

fn main() {
    println!("Named item: {}", Info::name());
    println!("Named item: {}", Bool3::name());

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