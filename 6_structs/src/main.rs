// Normal
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like
struct AlwaysEqual;

// This won't compile as it requires a "lifetime parameter"
// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

#[derive(Debug)] // using this for printing structs
struct Rectangle {
    width: u32,
    height: u32,
}

// using struct as a datatype
// we are borrowing the `rect1` as main() has the ownership of it and can use it later
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// methods are different from functions
// functions defined in here have their first parameter as `self` (as classes in python)
impl Rectangle {
    // these are called associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // initializing data
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    // mutate on values
    user1.active = false;

    // struct update
    let user2 = User {
        active: true,
        ..user1 // this must come last
    };

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area of rectangle: {}", area(&rect1));

    // this won't compile (struct doesn't implement `Display` trait)
    // println!("rect1 is {}", rect1);

    // but this will work
    println!("rect1 is {:#?}", rect1); // {:#?} means output format `Debug` with pretty-print

    // we can use another macro suitable for debugging
    dbg!(&rect1);

    // we can also use it inline like this
    let rect2 = Rectangle {
        width: 40,
        height: dbg!(rect1.height * 10),
    };

    // we can simply call a function on any struct type like this
    println!("area computed through method: {}", rect2.area());

    // we can also call/write associated functions like below
    dbg!(Rectangle::area(&rect2));
}
