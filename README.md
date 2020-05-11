# Learn Rust

Code and notes that I wrote while following along [The Rust Book][].

[the rust book]: https://doc.rust-lang.org/book/title-page.html

## Terms

- Arms
  - Blocks of code in a `match` or `if` segment
- Scalar Types
  - A type representing a single value (i.e. integers, floating point numbers,
    booleans, and characters).
- Compound Types
  - Types that can group multiple values into one type. Two primitive types are
    tuples and arrays.
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
  - Because a pointer is a known, fixed size, the pointer can be stored on the stack
  - When you want the actual data, the pointer must be followed to the data
- Owner
  - Each heap value has a variable called its owner
  - When the owner goes out of scope, the value is dropped (memory is free'd)
- Move
  - When a heap value is moved from one variable to another (ownership is moved)
  - Does not occur on types that implement the `Copy` trait
- Borrowing
  - When a function parameter uses a reference to a heap value. This prevents a `move`, causing
    the value to be unusable in the calling function.
- Structs
  - A named type that contains different named fields with types

## Concepts

- Blocks of code (i.e. things inside `{ }`) evaluate to the last _expression_
  in them.
- A type that has the `Copy` trait can have a value assigned from one variable to another,
  and the initial variable is still usable after the assignment.
  - If the type has any of its parts implement the `Drop` trait, then using the `Copy`
    trait is a compile error.
  - Integer, boolean, floating point, character types, and tuples if all containing types
    that have its types implement `Copy` are `Copy` common `Copy` types.
- Using types with `Copy` as an argument will allow using that variable after the function invocation
  in the calling function.
- Using types without `Copy` as an argument will take that variable out of scope for the remainder
  of the calling function.
