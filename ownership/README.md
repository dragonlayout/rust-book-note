#### What is ownership

> memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

#### Ownership rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
