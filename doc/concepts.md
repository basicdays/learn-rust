# Concepts

- Blocks of code (i.e. things inside `{ }`) evaluate to the last _expression_ in them.
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
- The module name of the crate root is `crate`. This is usually `main.rs` or `lib.rs`
- Idiomatic `use` paths
  - functions: path should be to module that contains function and not the function directly
  - structs, enums, etc.: specify full path
- elision rules
  - implicit rules for determining timelines for function/method inputs and
    outputs (rust 2018)
    1. all inputs get their own timeline
    2. if there is only one input, the output will get the same timeline
    3. if there are multiple input params, and one is `&self`, the output
       will get the same timeline as `&self`
- Box pointers
  - Stores data on the heap, rather than the stack.
  - Usually used when:
    1. When you have a type whose size can’t be known at compile time and you
       want to use a value of that type in a context that requires an exact size
    2. When you have a large amount of data and you want to transfer ownership
       but ensure the data won’t be copied when you do so
    3. When you want to own a value and you care only that it’s a type that
       implements a particular trait rather than being of a specific type
- Object-safe trait
  - Trait objects requires object-safety
  - The following are required for all methods in a trait:
    1. The return type isn't `Self`
    2. There are no generic type parameters
