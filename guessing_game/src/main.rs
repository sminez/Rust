extern crate rand;      // As 'rand' is declared as a dependency is Crate.toml this line allows us to
                        // make use of it. This also does the equivalent of a 'use rand'
use std::io;            // This is importing the io library from the std library
                        // NOTE: Python: from x import y --> Rust: use x::y;
use std::cmp::Ordering;
use rand::Rng;          // In addition to the 'extern crate rand' call we also need to make sure
                        // that Rng is in scope for a method call below. NOTE: This is because
                        // 'methods' are defined on things called 'traits' and for a method to work
                        // the corresponding trait needs to be in scope.

// Here we are defining the main function. As with C, you MUST have a main function as an entry
// point into the program. NOTE that here as no arguments or return values are given, this function
// implicity returns an empty tuple.
fn main() {
    // println!() is a 'macro' not a function. This is denoted through the '!' character.
    println!("Guess the number!");

    // This is the method call mentioned above. The gen_range() function works the same way as a
    // Python range: lower <= x < upper.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    // The 'loop' keyword will create an INFINITE loop inside its braced scope!
    loop {
        println!("Please input your guess: ");
        /*
        This is a variable binding (all of the form let foo = bar;)
        NOTE: Rust 'variables' are immutable by default like in Haskell, so they don't vary!
        The 'mut' keyword makes this a mutable reference.
        --> The String::new() part is calling an 'associated function' of the String type from the
            standard library. (A 'static method' in other languages). It is the '::' operator that
            denotes this. (Think of it as a dotted method in Python?)
        --> The function being called is new() which creates a new [empty] String (NOTE that this
            is NOT an object as in Python! Rust is more comparable to C.)
            The new() function is found on many types as a standard way of creating a new value.
        */
        let mut guess = String::new();

        /*
        Now that we are 'using' std::io we can call the stdin() function.
        --> We could have written std::io::stdin() instead of having the 'use' statement at the top.
        Here we call the read_line() method on a handle to the terminal Standard Input and passing it
        the argument '&mut guess'.
        --> NOTE: Here we are passing a mutable REFERENCE to the guess String we bound earlier. This
                  reduces copying of data and allows multiple areas of code to interact with the same
                  variables (Though these interactions are made safe by the Rust compiler.)
            .read_line() is taking a string from Standard Input and storing it in the memory location
            REFERENCED by our guess variable.
        The next two lines of the program together with this one actually form a single LOGICAL LINE
        when compiled (note the lack of sei-colons!). We could write the whole thing on a single line
        but Rust allows us to split these chained method calls with arbitrary whitespace to improve
        readability. (The whitespace is NOT dynamic as in Python and Haskell!!!)
        --> NOTE: The next two lines do the following: .ok() takes the return value (Result) of
                  .read_line() and assumes that no errors were raised, throwing away any error
                  information. .expect() takes the value of .ok() and if that value is NOT a successful
                  one it crashes the program using the 'panic!' macro, displaying the message given to
                  it as an error.
        */
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line...");
        /*
        As we are reading in 'guess' as a String but want to compare it to a numerical value we
        need to 'shadow' or original value with a new one. Here the ': u32' is a specific Type
        Annotation of a 32bit unsigned integer. This is provided so that Rust knows what to do with
        or original String. '.trim()' is a method on Strings to remove any whitespace: we need it
        to remove the newline character captured when the user hits return. '.parse()' will parse
        a string into some sort of number: this is why we need the Type Annotation -> to specify
        WHAT type of number we want the result to be.
        This time, instead of replacing the error with our own program crashing panic! we use a
        'match' statement to specify what is allowed and how to handle the error. In this case, we
        allow any valid value is passed on (the num name is simply the name for the pattern, it is
        the Ok() syntax that defines this as a valid value for the parse() function) and any errors
        are silently ignored.
        NOTE: This looks almost EXACTLY like Haskell pattern matching! Note the need for braces and
        a final semi colon but other than that we are giving an input (like a Haskell Function
        definition) and specifying what to do in the potential patterns. We even have '_' as a
        catch-all pattern.
        --> As with Haskell we need to ensure that we specify ALL possible patterns.
        */
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue,
        };

        // Here the println! macro we make us of string formatting similar to Python3's .format()
        println!("You guessed {}", guess);

        /*
        The 'cmp' method can be called on anything that can be compared (NOTE: Apparently this is
        similar to Haskell Type Classes?) and takes a REFERENCE to the thing we want to compare
        against as an argument, returning an Ordering (the 'use' from earlier). This Ordering
        is actually an 'enum', short for enumeration, which take the general form of:
            enum Foo {
                Bar,
                Baz,
            }
        NOTE: With this definition, anything of Type Foo is either a Foo::Bar or a Foo::Baz
        We then use a 'match' statement to determine which of the three Orderings we have obtained
        from 'cmp': Less, Greater or Equal. The match statement executes the selected code based
        on the value of its argument.
        NOTE: You can execute a block of code rather than a single statement by enclosing it in
        braces as with our Ordering::Equal case.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Ding ding ding! You win!");
                break;
            }
        }
    }
}
