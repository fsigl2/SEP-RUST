// Uncomment the following line to ignore dead code warnings:
#![allow(dead_code)]

// GIT:
// If you never used git before, try to use it during this exercise!
// Check out the first four chapters of this very short tutorial:
// https://kbroman.org/github_tutorial/
// Then start a new repo and do the usual git routine of add, commit, push
// Do the routine everytime you make a meaningful change, i.e. implement one function

// After you implemented a function, test it by using 'cargo test' to run the tests at the bottom of this file
//
// If you're stuck, check out the official Rust tutorial book and/or Rust by Example:
// https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html
// https://doc.rust-lang.org/rust-by-example/
//
// If you want to additionally test your solutions, see below the exercises

/// Ex.2i
fn var_decl() {
    // Add the missing keyword
    let x = 5;
    println!("x = {x}");
}

/// Ex.2ii
fn data_types() {
    // Replace ??? with fitting data types
    let a: i32 = 7;
    let b: &str = "Hello World";
    let c: bool = true;

    // Use if...else to print b if c is True or a if c is not True
    if c == true {
        println!("{b}");
    } else {
        println!("{a}");
    }
}

/// Ex.2iii
fn mutability() {
    // Add the missing keyword
    let mut x = 5;
    println!("{x}");

    x = 3; // Don't change this line!
    println!("{x}");
}

/// Ex.3a
// Add another parameter y of the correct type and the correct return type
fn add(x: i32, y: i32) -> Option<i32> {
    return Some(x+y);
    // Return the sum of x and y (you can do it two different ways, but one is more idiomatic!)
}

/// Ex.3b(i)
// Define a function 'fac_while', to do the following:
// The function takes a natural number n as input and returns a natural number
// Use a while loop to calculate the factorial of n
// The factorial n! of n is defined like so:
//   n! = n * (n - 1) * (n - 2) * ... * (n - (n - 1)) if n >= 1
fn fac_while(mut n: i32) -> Option<i32> {
    let mut res: i32 = 1;
    while n > 1{
        res *= (n);
        n -= 1;
    }
    return Some(res);
}

/// Ex.3b(ii)
// Use a for loop to write a 'fac_for' function, which does the same thing as your 'fac_while' function
fn fac_for(mut n: i32) -> Option<i32> {
    let mut res: i32 = 1;
    for mut i in 1..=n {
        res *= i;
    }
    return Some(res);
}

/// Ex.3b(iii)
// Use recursion to write a 'fac_rec' function, which does the same thing as your 'fac_while' function
// Try to use the match construct instead of if...else:
// https://rust-book.cs.brown.edu/ch06-02-match.html
fn fac_rec(mut n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * fac_rec(n-1),
    }
    
}

/// Ex.4
// Solve this exercise by reading the Rust documentation:
// https://doc.rust-lang.org/std/vec/struct.Vec.html
fn vec_basics() -> Vec<i32> {
    //normal array generation (cant change size)
    let arr: [i32; 5] = [1,2,3,4,5];

    //ways to generate a vector array
    let mut v = Vec::new();
    for n in arr {
        v.push(n);
    }

    let mut v2 = Vec::from(arr);

    let mut v3 = arr.to_vec();

    //appending to the vector
    v.push(6);
    v.push(7);
    v.push(8);

    //removing last object
    v.pop();

    //remove number at index
    v.remove(3);

    //get lenght of vector and print it
    println!("{}", v.len());

    // Return v
    v

}


//////////////////////
// END OF EXERCISES //
//////////////////////

// You don't need to write anything into the main function for this exercise
// You can use it to additionally test your implementation like so:
//    - use your functions in main()
//    - run your program with 'cargo run'
//    - observe the output
fn main() {
    println!("Hello Rust SEP!");
}

// You can add more tests below here, which you can then execute with 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex3a_test() {
        let res = add(5, 10);
        assert_eq!(res, 15);
    }

    #[test]
    fn ex3bi_test0() {
        let res = fac_while(0);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3bi_test1() {
        let res = fac_while(1);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3bi_test5() {
        let res = fac_while(5);
        assert_eq!(res, 120);
    }

    #[test]
    fn ex3bii_test0() {
        let res = fac_for(0);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3bii_test1() {
        let res = fac_for(1);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3bii_test5() {
        let res = fac_for(5);
        assert_eq!(res, 120);
    }

    #[test]
    fn ex3biii_test0() {
        let res = fac_rec(0);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3biii_test1() {
        let res = fac_rec(1);
        assert_eq!(res, 1);
    }

    #[test]
    fn ex3biii_test5() {
        let res = fac_rec(5);
        assert_eq!(res, 120);
    }

    #[test]
    fn ex4_test() {
        let res = vec_basics();
        assert_eq!(res, vec![1,2,3,5,6,7]);
    }
}
