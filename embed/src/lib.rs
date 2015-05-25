use std::thread;

/*
This is an instruction to the Rust compiler to NOT change the compiled name of the function so
that external processes know how to call it.
NOTE: The additional keywords infront of the function definition do the following:
    pub    -> allows the function to be called from OUTSIDE of this module.
    extern -> that call can be made from C (!)
    NOTE: See the Cargo.toml file for additional changes needed for this to work!
This is being compiled to a 'shared object' library that we then use as an external extension in
embed.py
NOTE: compile using 'cargo build --release'
*/
#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..5_000_001) {
                _x += 1
            }
            println!("done!");
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
}

#[no_mangle]
pub extern fn double_me(x: i32) -> i32 {
    x * 2
}
