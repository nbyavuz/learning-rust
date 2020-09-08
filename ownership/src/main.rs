fn main() {
    let string_move = String::from("String move");

    takes_ownership(string_move);

    // println!("{}", string); That will give errror because of move

    let string_clone = String::from("String clone");

    takes_ownership(string_clone.clone());

    println!("{}", string_clone);

    let x = 5;

    makes_copy(x); // Copy

    println!("{}", x);

} // string_move is already dropped,
  // string_clone and x drops

fn takes_ownership(some_string: String) -> String {
    some_string
} // drop

fn makes_copy(some_integer: i32) -> i32 {
    some_integer
} // no drop
