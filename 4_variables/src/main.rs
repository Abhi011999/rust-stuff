// a constant
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;

    //* Scoped declaration */
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //* Throws runtime error */
    // let y: u8 = "256".parse().expect("Not a number");
    // println!("y: {y}");

    //* Throws compile time error */
    // let _z = 256u8;

    let z = 98_222; // Same as 98,222 (just to be more readable)
    println!("z: {z}");

    let z = 0xff; // Hex
    let z = 0o77; // Octal
    let z = 0b11010_1111; // Binary
    let z = b'R'; // Byte

    let z = 2.0; // Floats

    //* Throws compile time error */
    // let err_mult = 2.0 * 2;

    let z = true; // Boolean

    let z = '👾'; // Character
    println!("z: {z}");

    tuples();

    arrays();

    statement_and_expressions();

    println!("returning function: {:?}", returning_function());

    println!("unit return: {:?}", unit_return());

    println!("{:?}", () == ());

    control_flow();
}

fn tuples() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");

    //* Throws compile time error */
    //* We can't take values like this */
    let i = 1;
    // println!("tup with index as var: {}", tup.i);
}

fn arrays() {
    // data allocated in stack rather than heap
    // not allowed to grow or shrink
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("a: {:?}", a);
    println!("a index 0: {:?}", a[0]);
}

fn statement_and_expressions() {
    let y = {
        let x = 3; // This is a statement
        x + 1
    }; // This is an expressions

    println!("y: {y}");
}

fn returning_function() -> i32 {
    5
}

fn unit_return() {}

fn control_flow() {
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num: {num}");

    // runs forever unless interrupt recieved
    // loop {
    //     println!("suiiiiiiii");
    // }

    // returning a value with break;
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {result}");

    // loop labels
    let mut counter = 0;

    'counting_up: loop {
        println!("counter: {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1
    }
    println!("end counter: {counter}");

    // for loop with range
    'for_loop: for number in 0.. {
        loop {
            if number == 10 {
                break 'for_loop;
            } else {
                println!("inside loop");
                break;
            }
        }
    }
}