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