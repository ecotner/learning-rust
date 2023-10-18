# HTTP server

Building a functional (but perhaps not super performant) web server.
The server uses the `HTTP/1.1` protocol.
It will have 3 components:
* TCP listener (sends/receives TCP data)
* HTTP parser (parses the HTTP messages)
* handler (handles the message data and decides what to do with it)

## Structs
* use `struct StructName {...}` to define struct
    * this holds a collection of data values
* use `impl StructName {...}` to define implementation of struct
    * this holds a collection of functions that can act on the data of the struct
    * functions can either be "methods" (take a `self` argument referring to a specific struct object) or "associated functions" (can be used without an initialized struct object, kind of like a static method)
        * beware that when declaring methods that if the `self` argument is not a borrowed reference, the struct object will be deallocated after the method exits. You can avoid this by making a reference using `&self` or `&mut self`.

## Strings
* distinction between `String` and `&str` types
    * first is a dynamic string allocated on the heap, the second is a "string slice"
    * string slice can be extracted from a `String` using `&string_var[from..to]`
        * `from` and `to` are integer indexes (zero-indexed)
        * can take slices from beginning with `[..to]` and until the end with `[from..]`
        * string slice contains reference to substring of a `String` and a length without copying data

## Enums
* can create enum with `enum EnumName {...}` where body is list of "variant" names
* automatically will be encoded as incrementing integers, but we can control that by manually assigning values
* variants of the enum can optionally contain different data types (e.g. `String`, `u64`, `bool`, etc) instead of the default
    * program will allocate memory for the largest variant because it doesn't know which one it will actually take on

### Option enum
* the `Option` enum is a special built-in enum that is used to deal with the absence of a `null`-like value in the rust language
* `Option<T>` has two variants; `None` and `Some(T)`, so that you can easily tell whether a value is `None` or not
* the `T` can be any type (this is an example of "generics", which we will learn more about later)

### Result enum
* built-in `Result` enum at the core of error handling in rust; there is no such thing as exceptions
* errors are either recoverable or unrecoverable
    * e.g. file not found would be recoverable because you might not want the program to crash; unrecoverable would be like an out of bounds index error
* variants of the `Result<T, E>` enum are either `Ok(T)` or `Err(E)`
* can extract the `Ok` value from the `Result` using `.unwrap()`
    * if an error is encountered, the program will exit

## Modules
* can create namespaces with specific scope by declaring modules
* use `mod module_name {...}` to declare
    * anything inside the `{...}` will be in the module
* can refer to types within the module using `module_name::TypeName`
* you can have submodules within parent modules
    * can refer to parent module using the `super` keyword; e.g. `super::other_module::OtherType`
    * submodules are private by default, so use `pub mod` to make them public
* can use `use` keyword to pull things into scope from other modules without explicitly referring to the module
* rust treats files named `module_name.rs` as if the code inside were a module with name `module_name`
    * you can import a module into another one using `mod module_name;`
    * nested directories/folders are also treated as modules
        * because a directory doesn't necessarily have any code associated with it, a special file called `mod.rs` can be placed in a directory to declare which objects/submodules are public (kind of equivalent to `__init__.py` in python)
        * you can also pull types from submodules into the scope of a higher module with `pub use submodule::Type`

## Loops
* rust has `while <condition> {...}` loop; if we want to loop forever, we can use `loop {...}`
* also supports `break` to exit a loop or `continue` to skip an iteration
    * can also "name" loops with a label (via an apostrophe like `'loop_name loop {...}`) and refer to specific loop when using `break/continue 'loop_name`
* also has python-like `for` loop capabilities; can iterate over ranges `n..m`, or arrays `[1, 2, 3, 4, ...]`, or other things that implement an `Iterator` interface?

## Tuples
* can be used to handle multiple objects at the same time
* allows functions to return more than one thing
* they have a fixed length; cannot grow or shrink in size
* elements can be heterogeneous (mixed types)

## Match expression
* can use an `if` statement to handle errors by using `if result.is_err() {...}`
* this can get unweildy for more general enums or multiple comparisons, so rust has a `match` functionality that makes this easier
* format is `match enum_var {...}`
    * body is comma-separated list of variant names followed by code to exectute; e.g. `VarName1(v1) => {...}, VarName2(v2) => {...}`
    * the `v1/v2` refer to variables that are wrapped by the variant
    * for a `res: Result` enum in particular, we might have `match res {Ok(val) => {...}, Err(e) => {...}}`
    * use `_` to refer to a variable you don't care about
* match also works on other types too besides enums
    * can match multiple criteria using a pipe `|`

## Arrays
* an array is a homogeneous collection of variables
* can be declared via a comma-separated list in square brackets `[v1, v2, ...]`
* the type declaration is `[T, n]` where `T` is the type contained by the array, and `n` is an integer representing the length of the array
    * array length must be known at compile time so we know how much memory to allocate on the stack for it
* for functions that work with arrays, it is too restrictive to require that we know how big an array is in the function argument, so we can use a reference via `&[T]` instead (this is called an array slice)

## Reading TCP connection
* in the `std::net` crate, there is a `TcpListener` struct that can listen on a socket using the `TcpListener::bind(addr)` method
* the listener returns a `(TcpStream, SocketAddr)` tuple; and we can use the stream to read raw bytes into a buffer with `stream.read(&mut buffer)`
* the buffer is a `[u8]` array; in order to convert this to UTF8, we can use `String::from_utf8_lossy(&buffer)` (we use the _lossy_ one to replace corrupted characters with � without throwing an error)

## Traits
* traits are a way to extend types that were defined elsewhere
* we can use the `TryFrom` trait to extend our `Request` object to allow us to convert our byte array into a `Request` struct
* define a trait with `trait TraitName {...}`
    * can use generics to specify optional types `trait TraitName<T> {...}` or specify in the body `trait TraitName {type T; ...}`
* can also define functions in the trait body that specify a _contract_ that any implementation of the trait must follow
    * can implement the trait with `impl TraitName for StructName {...}`; this MUST define a concrete implementation of the functions specified in the trait definition

## Custom errors
* can define custom errors by implementing the `Error` trait for a specific struct like `impl Error for CustomErrorName {}`
    * implementing `Error` will also require that you implement the `Display` and `Debug` traits as well
    * these two traits require defining a `fmt` method that basically takes a formatter object and uses it to format a string (use the `write!` macro for this)
* can convert between error types by implementing a `From<E>` trait for the error type you want to convert to
    * then you can use the `?` operator to auto-convert the error type and return the new one if one is produced, otherwise return the value contained in the `Result`
        * e.g. `Result<int, Error>(5)?` will return 5 if the variant is `Ok`, otherwise will break control flow early and return an `Error`
    
## Iterators
* rust supports the concept of iterators, which are structs that implement a `next()` method that returns the next element in the iterator, or `Option::None` if there are no more elements
* can iterate over an iterator using a for loop: `for val in iterator {...}`
    * if we want to get the index of the element we can also do `for (i, val) in iterator.enumerate() {...}`
* to iterate over the characters of a string, we can call `string_var.chars()`

## If let
* rust has some syntax that will let you do pattern matching in an `if` statement: `if let pattern = variable {... body can use <pattern> variable ...}`
* is useful if you need to do a `match` but only care about some of the variants

## Lifetimes
* in languages that allow pointers, it is sometimes possible to cause a "use after free" error, where some part of your code tries to read data on the heap that has been deallocated
* in a garbage-collected language, this never happens because as long as there is a reference to some variable, it will not be collected
* rust doesn't have a garbage collector, but can ensure this never happens at compile time by analyzing the "lifetimes" of your variables to ensure that references to a variable don't "live" longer than the variable itself
* the compiler can't figure this out on its own though, so we have to provide "lifetime annotations"
    * annotations are specified with an apostrophe followed by the lifetime name, e.g. `'my_lifetime` (many people use simple, single-character names like `'a` or `'b` in their code)
* you can annotate variables with `&'a var`, and structs, traits, and functions, etc with `<'a>`
    * implementations of structs that have a lifetime annotated must annotate the `impl` keyword: `impl<'a> Trait for Struct<'a> {...}`
* what this does is tell the compiler that specific variables are related to each other in terms of memory, NOT that we are specifying explicit lifetimes

## Hashmap and vector
* rust has some cool higher-level collection data structures that are part of the `std::collections` crate
* there is a `HashMap<K,V>` that creates a hash map from type `K` to value `V`
    * has some useful methods like `.get` and `.entry`
* there's also a `Vector<T>` that creates a dynamic-size array of type `T`
    * can use `.push` to append new elements
    * there is a convenience macro `vec![a, b, c, ...]` that will automatically create a vector with elements `a`, `b`, `c`, etc.

## Attributes (particularly the `derive` one)
* attributes are some kind of metadata that is used by the compiler to do special things
* we can prepend a data structure with `#[derive(Debug)]` on the line above it in order to implement the `Debug` trait for the struct automatically
* then we can use the `dgb!(...)` macro to easily print debug information
* if the struct has any members that themselves need to implement `Debug`, we can sprinkle the attribute across the structs that need it
* notice that the `#[...]` is missing the bang like in the `#![allow(dead_code)]` we used earlier to silence the linter
    * the bang means that the attribute applies to everything the attribute is declared _within_, while attributes without a bang apply only to whatever immediately follows

## Copy and clone types
* If you want to modify the data in a reference without mutating the original data, you need to "move" it somehow
* If the data is a simple type that lives on the stack like an integer, it can be _copied_, but if it is a complex type that lives on the heap, it must be _cloned_ (basically a deep copy)
* can do this by implementing the `Copy` and/or `Clone` traits on a struct
* beware though that copying/cloning data can potentially be wasteful if it is done haphazardly; often it is better to keep track of references to data instead of copying the values

## Dynamic and static dispatch
* the `send` function we implemented earlier accepts a `TcpStream` as an argument and writes some data to it, but what if we wanted to generalize the function so that it accepted any struct that implemented the `Write` trait?
* one way to do that is replace `TcpStream` with `dyn Write`, where the `dyn` refers to "dynamic dispatch"
    * at runtime, the program will look up the implementation of the `write!` function for the type that we passed in (using something as a v-table)
    * this adds a bit of overhead due to the lookup, and can slow things down if this is done in a hot path
* another way to do it is to replace `TcpStream` with `impl Write`; this utilizes "static dispatch"
    * now multiple functions will be created at compile time and will be called appropriately at runtime
    * this can be much faster, but comes at the cost of a larger binary size

## Custom traits
* we can create a custom trait using `trait TraitName {...}`
* in the body we put the function signatures of the methods we want implementors of the trait to implement
* if we want a default implementation, we can put that in the body of the methods, and then trait implementations can optionally overwrite them