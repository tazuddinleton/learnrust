## Quick overview of cargo commands

Use `cargo new <name>` to create new project with cargo. It will create project with a git repository already initialized for you. But it won't create repository if you run it from existing repo.

Use `--vcs=<your prefered vcs or none>` flag with `cargo new` to choose vcs of your choice.

Use `cargo build` to build the program.

Use `cargo run` to build and run the program.

Use `cargo build --release` for production build.


Use `cargo doc --open` for documentations of dependencies used 


## Ownership, Copy and Move of values, Borrow check

In Rust each value have a single owner, in other words each value has a single location that ultimately de-allocates the value. That location is usually a scope. If you reassign a value to a new variable then the owenership moves to new variable, if its type does not implement `copy` trait. 

There is a special trait called `Copy` that a type can implement to make itself un movable. Meaning that when a type T implements the `Copy` trait its values does not move to new location when re-assigned. Almost all primives in rust have this trait. 

Rust checks ownership at compile-time using it's borrow checker. 

## Borrowing
In rust you can lend a value to other instead of completely give up the ownership using `References`. 

There are two kinds of references
- Shared reference `&T`
- Mutable reference (exclusive) `&mut T`

Rust compiler assumes that shared references are not mutable that no matter how many references are there, no one is going to change the value. In case of mutable references however compiler assumes that it is going to change. You can not have both references at the same time.


