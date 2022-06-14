// fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    // move occurs because `f.name` has type `String`, which does not implement the `Copy` trait
    let _name = f.name;

    // ONLY modify this line
    // f can't be used, because partial(f.name) of it is moved
    println!("{}, {}, {:?}", _name, f.data, f);
}
