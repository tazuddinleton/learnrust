fn main() {
    let x1 = 42;
    let y1 = Box::new(84);

    {
        let z = (x1, y1);
    }

    let x2 = x1;
    // This is going to fail
    // let y2 = y1;


    // let input = 1;
    // let mut output = 0;
    // noalias(&input, &mut output);


}


fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }

    if *input != 1 {
        *output = 3;
    }
}

