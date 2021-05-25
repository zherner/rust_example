// test
fn main() {
    println!("{} days", 31);
    println!("{2} {3} {1} {0}", 4, 3, 1, 2);
    println!("{b} {a}", b = "bob", a = "ann");
    println!("{} of {:b} people know binary, the othe half doesnt", 1, 2);
    println!("{number:>width$}", number = 1, width = 7);
    println!("{number:>0width$}", number = 1, width = 7);

    let pi = 3.141592;
    println!("Pi is about {:.2}", pi);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
