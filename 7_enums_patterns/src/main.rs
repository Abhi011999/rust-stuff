// enums are like custom datatype
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let ipV4 = IpAddrKind::V4;
    let ipV6 = IpAddrKind::V6;

    // calling via enums
    route(ipV4);

    // use of enums with structs
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // use of enums directly with associated data
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    // multiple params
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr3::V4(127, 0, 0, 1);

    // a more complex enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // we can define methods on this and then call it
    impl Message {
        fn call(&self) {
            dbg!("in call()");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // use of generic type enums
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // using incompatible types
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this won't compile
    // let sum = x + y;

    // but this will
    let sum = x + y.unwrap();
    dbg!(sum);

    // using match control flow
    // (this type of pattern matching is used extensively in rust's own libs because it's useful in many situations)
    let coin: Coin = Coin::Nickel;
    dbg!(match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    });

    // default action with underscore
    let dice_roll = 5;
    dbg!(match dice_roll {
        3 => dbg!("three"),
        7 => dbg!("seven"),
        _ => dbg!("other"),
    });

    // `if let` control flow
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // one more use-case with enums
    let mut count = 0;
    if let Coin::Quarter = coin {
        println!("state")
    } else {
        count += 1;
    }
    dbg!(count);
}
