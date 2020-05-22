# Scratch

## Hashmap Macro

https://crates.io/crates/maplit
https://github.com/DanielKeep/rust-grabbag/blob/master/grabbag_macros/src/lib.rs#L3-L64
https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
https://stackoverflow.com/questions/24276617/why-does-this-rust-hashmap-macro-no-longer-work

```rust
#[macro_export]
macro_rules! map(
  { $T:ident, $($key:expr => $value:expr),+ } => {
    {
      let mut m = $T::new();
      $(
        m.insert($key, $value);
      )+
      m
    }
 };
)
```

```rust
macro_rules! foo(
  {$T:ident} => { $T; };
)

struct Blah;

#[test]
fn test_map_create() {
  let mut bar = foo!{Blah};
}
```
