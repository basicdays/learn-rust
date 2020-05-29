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
