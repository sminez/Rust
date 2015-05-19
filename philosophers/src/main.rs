use std::thread;                // Remember: in Rust, use brings a name into scope.
use std::sync::{Mutex, Arc};    // Note the braces to import multiple things from a single library.
/*
Here we are defining a struct: a collection of named data fields with associated methods on the
data and associated functions (Like a static method in Java?).
    NOTE: associated functions are called using the double colon on the struct name -
        struct_name::function_name()
    While methods can be accessed using dot notation on an INSTANCE of a struct -
        struct_INSTANCE.method_name()
*/
struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

/*
An 'impl' block allows us to define things on a 'struct'. Here we are defining an associated
function called 'new'. We know that it is an associated method and not a method as all methods
contain a reference to 'self' as they are operating on the data within the struct itself.
*/
impl Philosopher {
    /*
    For the 'new' function we take reference to another string, copy that as a NEW string and
    store that as the name field of our Philosopher. NOTE: This means that we ARE creating a
    new string and storing it separately unlike in Python where we simply attach a new label
    to an existing value.
    NOTE: the '-> Philosopher' section of the function definition is providing a return value but
    we don't explicitly return the Philosopher that we create: we only define it. Rust is an
    expression based language, meaning that [almost] everything is an expression that returns a
    value. In a function, the last expression is automatically returned: so here we return our
    newly created Philosopher.
    NOTE: Semi-colons in Rust are used to mark the end of an EXPRESSION not a logical line of code.
    */
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
    /*
    Here we define the METHOD 'eat' which takes data from the struct and uses it. In addition, we
    are also accessing data from the Table struct, namely the Vec! of Mutexes. The lock() function
    simply aquires the lock on the mutex selected (via the indices self.left and self.right) and
    gives that hread access to the data inside (and empty tuple). If the lock is not available, the
    thread will block until it becomes available. The Mutex is then automatically released when
    _left and _right go out of scope. unwrap() takes the Option type returned by the Mutex and
    returns the value inside.
    NOTE: The underscore on _left and _right is informing the Rust compiler that we are throwing
    away the value of this expression (remember, in Rust pretty much everything is an expression
    that returns a value) in order to stop it giving us a compile error for defining a value that
    isn't used.
    */
    fn eat (&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} sat down and started eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating and has left the table.", self.name);
    }
}

/*
A Mutex is a way of controlling concurrency: only one thread can access its contents at a time. We
are using an empty tuple as the contents of the Mutex in this case as all we care about is the fact
that a single thread can have it.
*/
struct Table {
    forks: Vec<Mutex<()>>,
}

/*
In the main function we now create 5 variable bindings with 5 Philosophers. [There is nothing magic
about using the name new() for our creation function other than convention and ease of use.]
NOTE: If we had not defined the new() function above we COULD have implemented done this instead:
    "let px = Philosopher {name: "Philosopher x's name".to_string() };"
While this is valid code in Rust (explicitly naming the fields of the struct and initialising them)
it leads to a lot of line noise and it is much preferred to define a new() function for all structs
that you create. [Readability counts!]
*/
fn main() {
    /*
    An Arc is an Atomic Reference Count: it allows us to to share 'table' accross multiple threads.
    The count goes up as we share it and drops back down as each thread finishes executing.
    */
    let table = Arc::new(Table {forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        ]});
    // The vec! macro creates a vector (or Vec<T>): a growable array
    let philosophers = vec![
        Philosopher::new("Socrates", 0, 1),
        Philosopher::new("Descartes", 1, 2),
        Philosopher::new("Kant", 2, 3),
        Philosopher::new("Hume", 3, 4),
        // Sun Tzu is left handed! This prevents deadlock...
        Philosopher::new("Sun Tzu", 0, 4),
    ];

    /*
    Here, 'handles' is a new binding of type "Vector of [...something...]". The _ is a type
    placeholder -> we are asking Rust to fill in the type information at compile time.
    The .into_iter() method takes a vector and turns it into an iterator that takes ownership of
    each Philosopher. NOTE: the iterator MUST take ownership of the Philosophers in order to pass
    them off to the threads.
    We then call 'map' on the iterator we just created which takes a closure as an arguments and
    calls that closure on each element of the vector in turn.
    Next, the thread::spawn function takes a closure as an argument and executes it in a new thread
    by making use of the 'move' keyword to indicate that the closure takes ownership of the value
    that it is capturing.
    The '.collect()' method gathers up the result of all of the map() calls and stores them as a
    collection. NOTE: this is why we needed to specify a return type of Vec<_>: the return type of
    thread::spawn is a handle to the thread.
    */
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        /*
        The .clone() method is what is incrementing the Arc on table. As before, we introduce a new
        binding to 'table' to shadow the original so we don't have to come up with a new name. This
        is OK as the new binding is only valid inside the current scope.
        */
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    /*
    Rust's for loop is the same as Python's: it takes an expression and an iterator and uses
    that expression on / as each element of the iterator.
    NOTE: .join() ensures that all of the threads have finished executing before we exit the main
          program.
          .unwrap() takes an option type and extracts the return value from it without copying it.
    */
    for h in handles {
        h.join().unwrap();
    }
}
