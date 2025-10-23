// Uncomment the following line to ignore dead code warnings:
// #![allow(dead_code)]

// If you're stuck, check out the official Rust tutorial book and/or Rust by Example:
// https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html
// https://doc.rust-lang.org/rust-by-example/

/// Ex.2a
/// DONE ??
// Fix the compiler error in this function
// Then fix the error in ex1a_test below
fn vec_77(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(77);
    vec
}

/// Ex.2b
/// DONE
// Fix the error only by reordering the lines
// Don't add, remove or change any lines
fn mut_borrow() -> Vec<i32> {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(7);

    x
}

/// Ex.2c
/// HOPEFULLY THIS IS WHAT THEY WANTED ME TO DO
// What needs to be changed in these two functions so that the code in the main function works?
// Don't change any lines in the main function!
fn get_char(mut s: String) -> char {
    s.chars().last().unwrap()
}

fn string_uppercase(s: &String) {
    let news = s.to_uppercase();

    println!("{news}");
}

/// Ex.2d
// What needs to be changed so the code in the main function works?
// There are multiple ways!
// For this you can change code in the main function as well #[derive(Debug)]
#[derive(Debug, Clone)]
struct Person {
    age: u8,
}

fn main() {
    // Ex.2c
    let s = "Rust is the best!".to_string();
    get_char(s);
    string_uppercase(&s);

    // Ex.2d
    let alice = Person { age: 42 };
    let bob = alice.clone();
    println!("Alice: {:?}\nBob: {:?}", alice, bob);
}

// You can add more tests below here, which you can then execute with 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;

    // Fix the errors here
    #[test]
    fn ex2a_test() {
    
        let mut vec0 = vec![11, 33, 55];
        let vec1 = vec_77(&mut vec0);
   // changed function calls below
   // this works since vec1 is mut borrow, and vec0 cant be immmut borrowed simultaneously 
        assert_eq!(*vec1, vec![11, 33, 55, 77]);
        assert_eq!(vec0, vec![11, 33, 55]);
    }

    #[test]
    fn ex2b_test() {
        let res = mut_borrow();
        assert_eq!(res, vec![42, 67]);
    }
}
