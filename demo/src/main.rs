use macro_examples::{NameFn, declare_variables, clear};

trait Name {
    fn name() -> String;
}

#[derive(NameFn)]
struct Info;

#[derive(NameFn)]
enum Bool3 { True, False, Unknown }

#[clear]
fn report_existence() {
    println!("I exist!");
}

fn main() {
    // report_existence();

    println!("Named item: {}", Info::name());
    println!("Named item: {}", Bool3::name());

    declare_variables! {
        a = 'a',
        b = 2,
        c = a,
        d, // will be defaulted to 0
        e = "e",
    }
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
}