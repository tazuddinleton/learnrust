fn main() {
    // println!("hello world");


    // let x = Box::new(10);
    // let y = x;
    // println!("x: {x}, y: {y}");

    let mut k = Box::new(10);
    
    println!("before change: k = {}", k);
    replace_with_84(&mut k);
    println!("after change: k = {}", k);
}

fn replace_with_84(s: &mut Box<i32>) {
    // let was = *s; // invalid
    
    let was = std::mem::take(s); // valid
    // *s = Box::new(41); // valid

    // let mut r = Box::new(84);
    // std::mem::swap(s, &mut r);

    // println!("r: {} ", r);
    // assert_ne!(*r, 84);
}