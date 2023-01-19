## Mutability
A variable in rust is by default immutable. If you want a mutable variable you have to use `mut` keyword when declaring it.
Like 
```
let mut x = 10;
x = 42;
```

## Shadowing
In rust you can declare variable with same name multiple times. The latter one will *shadow* prior one. Like below
```
let foo = 10;
let foo = 42;
{
    let foo = foo * 2; // here the value is 84!
}
// here the value is 42 again!
```
At the end the value of `foo` will be `42` and it is totally valid rust code. 


