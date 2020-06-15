# Terms

- Arms
  - Blocks of code in a `match` or `if` segment
- Scalar Types
  - A type representing a single value (i.e. integers, floating point
    numbers, booleans, and characters).
- Compound Types
  - Types that can group multiple values into one type. Two primitive types
    are tuples and arrays.
- Parameters
  - Variables defined in a function signature
- Arguments
  - Values passed into a function during invocation
- Expression
  - Instructions that evaluate to a resulting value
- Statement
  - Instructions that perform an action and don't return a value
- Stack
  - Stores values in the order it gets them (last in, first out)
  - Data stored on the stack must be a known, fixed size
  - Adding data is called "pusing onto the stack"
  - Removing data is called "popping off the stack"
- Heap
  - Stores unknown sized data (can grow over time)
  - The OS will allocate a section of memory and return a pointer
  - This is called "allocating on the heap" or just "allocating"
  - Pushing values onto the stack is not considered allocating
  - Because a pointer is a known, fixed size, the pointer can be stored on
    the stack
  - When you want the actual data, the pointer must be followed to the data
- Owner
  - Each heap value has a variable called its owner
  - When the owner goes out of scope, the value is dropped (memory is
    free'd)
- Move
  - When a heap value is moved from one variable to another (ownership is
    moved)
  - Does not occur on types that implement the `Copy` trait
- Borrowing
  - When a function parameter uses a reference to a heap value. This
    prevents a `move`, causing the value to be unusable in the calling
    function.
- Structs
  - A named type that contains different named fields with types
- Module system
  - Made up of packages, crates, modules, and paths
- Package
  - One or more crates that provide functionality
- Crate
  - A binary or library within a package
  - A package with `src/lib.rs` has a library crate
  - A package with `src/main.rs` has a binary crate
- Crate Root
  - Source file that the compiler starts with in a crate
- Trait Bound
  - When a generic type parameter is bound to specific traits
- Blanket Implementation
  - Conditionally applied trait on any type that satisfies trait bounds
    ```
    impl<T: Display> ToString for T {
      // --snip--
    }
    ```
- LifeTime
  - Scope for which a reference is valid
- consuming adapters
  - Methods that call `next` on an iterator. They will take ownership of
    the iterator.
- pointer
  - A variable that contains an address in memory. This address "points at"
    some other data.
- reference pointer
  - Indicated by a `&` symbol and borrow the value they point to.
- smart pointer
  - Act like a pointer, but have additional metadata and capabilities.
  - May _own_ the data it points to
- Object-oriented languages
  - Object-oriented programs are made up of objects. An object packages both
    data and the procedures that operate on that data. The procedures are
    typically called methods or operations.
- Encapsulation
  - The implementation details of an object aren’t accessible to code using
    that object
- Inheritance
  - A mechanism whereby an object can inherit from another object’s definition,
    thus gaining the parent object’s data and behavior without you having to
    define them again.
- Polymorphism
  - The ability to substitute multiple objects for each other at runtime if
    they share certain characteristics.
- Bounded Parametric Polymorphism
  - Rust uses generics to abstract over different possible types and trait
    bounds to impose constraints on what those types must provide.
- Trait object
  - A trait object points to both an instance of a type implementing our
    specified trait as well as a table used to look up trait methods on that
    type at runtime.
- Dynamic dispatch
  - The compiler can't tell at compile time which method you're calling. The
    compiler will emit code that at a runtime will figure out which method to
    call.
- Irrefutable Pattern
  - Patterns that will match for any possible value
  - e.g. `let x = 5;`
  - The pattern `x` will match anything and will not fail to
    match.
  - Required in function parameters, `let` statements, and `for` loops
  - e.g. `let Some(x) = some_option_value;` is invalid
- Refutable Pattern
  - Patterns that can fail to match for some possible value
  - e.g. `if let Some(x) = a_value`
  - Allowable in `if let` and `while let` expressions
  - Match arms must use refutable patterns, except for the last arm which should
    match any remaining values with an irrefutable pattern
