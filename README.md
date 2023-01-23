# Martta
A Programming language with Scala inspired syntax and static typechecking.


## Examples
If you are familiar with Scala or Rust, Martta should be quite familiar. Here's a small snippet of its syntax:
 ```
let baz: int = 20;
let foo: Int = |a| => { a + baz };

println(foo(10)) // 30
```



## Build
Make sure you have `rustc 1.66.0`at least. 

    git clone https://github.com/luisvgs/martta.git && cd martta && cargo build
