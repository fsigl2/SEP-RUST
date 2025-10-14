// Uncomment the following line to ignore dead code warnings:
// #![allow(dead_code)]

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
    let a: ??? = 7;
    let b: ??? = "Hello World";
    let c: ??? = true;

    // Use if...else to print b if c is True or a if c is not True
}

/// Ex.2iii
fn mutability() {
    // Add the missing keyword
    let x = 5;
    println!("{x}");

    x = 3; // Don't change this line!
    println!("{x}");
}

/// Ex.3a
// Add another parameter y of the correct type and the correct return type
fn add(x: i32, ???) -> ??? {
    // Return the sum of x and y (you can do it two different ways, but one is more idiomatic!)
}

/// Ex.3b(i)
// Define a function 'fac_while', to do the following:
// The function takes a natural number n as input and returns a natural number
// Use a while loop to calculate the factorial of n
// The factorial n! of n is defined like so:
//   n! = n * (n - 1) * (n - 2) * ... * (n - (n - 1)) if n >= 1

/// Ex.3b(ii)
// Use a for loop to write a 'fac_for' function, which does the same thing as your 'fac_while' function

/// Ex.3b(iii)
// Use recursion to write a 'fac_rec' function, which does the same thing as your 'fac_while' function
// Try to use the match construct instead of if...else:
// https://rust-book.cs.brown.edu/ch06-02-match.html

/// Ex.4
// Solve this exercise by reading the Rust documentation:
// https://doc.rust-lang.org/std/vec/struct.Vec.html
fn vec_basics() -> Vec<i32> {
    let arr: [i32; 5] = [1,2,3,4,5];

    // Create a Vec with the same elements that are in 'arr' (there are multiple ways)

    // Add 6,7,8 to the end of v

    // Remove the last number in v

    // Remove the number at index 3

    // Calculate the length of v and print it

    // Return v
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
