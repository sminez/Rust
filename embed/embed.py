'''
This is an example of how to call a Rust compiled 'C extension' that has been written in Rust!
See lib.rs in the /src directory for the corresponding Rust code.

Look into how to execute individual functions from within an .so file. (FFI section: p106)
'''
from ctypes import cdll

# This loads the library compiled from Rust and then calls the code within Python.
# This could MASSIVELY speed up computation!
lib = cdll.LoadLibrary("target/release/libembed.so")
lib.process()

print("Done!")
