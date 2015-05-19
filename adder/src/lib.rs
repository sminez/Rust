// This file contains a short example of how to implement simple unit style testing in an idiomatic
// way via an inner test module.


/*
// This attribute marks this as a test function to be run when we use cargo test.
// A test passes if it doesn't panic! and fails if it does.
#[test]
fn it_works() {
    // The assert! macro does nothing if the argument is true and panic!s if it false.
    assert!(true);
}

#[test]
fn all_things_equal() {
    // assert_eq! checks that both arguments are the same
    assert_eq!(false, false);
}

#[test]
// The #[should_panic] attribute inverts the normal test failure check: making panic! a pass.
#[should_panic]
fn failing_is_ok() {
    assert_eq!("yes", "no");
}

#[test]
// #[should_panic] is dangerous as it can cover up an unexpected panic! as well as the one we are
// expecting. This syntax allows us to specify what we are expecting to happen.
#[should_panic(expected = "assertion failed")]
fn assertion_failure() {
    assert!(false);
}
*/

// The above are some simple quick test semantics examples but are not idiomatic Rust. See below
// for an example of the CORRECT way to do it.

pub fn add_two(a: i32) -> i32 {
    a + 2
}

/*
A #[cfg(test)] attribute ensures that the following code is only compiled when we are using
'cargo test' to compile.
We are also defining a 'module' (mod) of code to group all of our test code together so that it is
hidden from a normal build AND speeds up compilation time.
*/
#[cfg(test)]
mod tests {
    /*
    As we are now in an inner module, the rest of our code is out of scope! use super::* is a quick
    'glob' to pull everything else in this file into scope.
    For small projects we could simply write 'use super::fn_name' for each function.
    */
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(4, add_two(2));
    }
}
