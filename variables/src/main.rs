fn main() {
    let mut x: i32;    
    x = 11;

    println!("value of x: {}", x);

    shadow();
}

fn shadow() {
    let foo = 10;
    let foo = 42;
    {
        let foo = foo * 2;
        println!("value of foo in the inner scope: {}", foo);
    }

    println!("value of foo in the outer scope: {}", foo);
}
