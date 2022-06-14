fn main() {
    let tuple_a = (1, 2, 3);
    let tuple_b = (4, 5, 6, 3, 5, 6, 1, 1, 10, 22, 10, 1, 2, 5, 8);

    println!("{}", tuple_a.0);
    println!("{}", tuple_b.1);
    // too long to print tuple_b
    // println!("too long{:?}", tuple_b);

    // one element tuples, the comma is required
    println!("one element tuple{:?}", (1,));
    println!("just a integer{:?}", (1u32));
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a, b) = pair;
    (b, a)
}
