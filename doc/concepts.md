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
