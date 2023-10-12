# Mars weight calculator

A simple CLI application that will calculate your weight on Mars.
Using this as an example to learn rust's basics.

## Some important things learned

* variables can be assigned with `let`
    * variables are _immutable_ by default; need to declare them _mutable_ with `let mut`
    * unless data type can be inferred somehow, also need to specify
* macros look like functions but end in a exclamation mark `!`
    * macros are bits of code that dynamically write new code
    * macro definitions and how they expand to new code can be very tricky
* functions can be defined using `fn function_name(arg1: arg_type, ...) -> return_type {...}`
* reading from stdin requires using the `io` crate from the rust standard library, which can be imported with `use std::io`
    * need to pass a _reference_ to a "string buffer" (a dynamically-sized string data structure) to the stdin `read_line` method so that the program knows _where_ to store the data being read in
* rust has pretty strict rules around "ownership" of data
    * each value is owned by a variable
    * when the owning variable passes out of scope, the value is deallocated
    * each value can only have ONE owner at a time
* references allow a reference to some data without taking ownership of it (known as "borrowing")
    * this allows using a variable in a function without destroying it once it goes out of scope inside the function
    * you can indicate a reference to a variable by prepending an ampersand: `&variable`
    * if you want to allow the reference to mutate the value, you must use `&mut variable` instead
    * you can have as many immutable references to a value as you want, but only a single mutable reference
        * this prevents all data races at compile time because only one thread at a time can mutate the referenced data
    * you can dereference a pointer using an asterisk: `*pointer`
* it's possible that some functions return errors; rust handles this by returning a `Result` type that can resolve to either `Ok` or `Err`
    * if the result is `Ok`, we can use `.unwrap()` to return the value
    * I assume we'll learn how to handle `Err` later