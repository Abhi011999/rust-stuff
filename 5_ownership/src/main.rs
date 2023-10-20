fn main() {
    // allocating s1 into memory
    let s1 = String::from("hello");

    // a shallow copy is created i.e. only pointer, length and capacity are copied
    // after that `drop` is called on s1 to free s1's memory
    let s2 = s1;

    // this won't compile because
    // we are trying to access s1 which is dropped & moved to s2
    println!("{s1}");

    // this will work because `i32` implements `Copy` trait
    // which makes a deep copy of `x` into `y` and doesn't call `drop`
    let x = 5;
    let y = x;
    println!("{x}");

    // this won't also work as we pass `s2` to other function
    // when that function goes out of scope, the moved value is also cleared
    // making `s2` void
    some_func(s2);
    println!("{s2}");

    // this will work as we use a referencing operator (&) on it
    // here, function will not have the ownership of `s1`
    // hence the value of `s1` will not be dropped
    let s1 = String::from("referencing");
    ref_func(&s1);
    println!("{s1}");

    // this won't work as we are trying to mutate the value of `s`
    // which is immutable by default
    let s = String::from("immutable borrow");
    change(&s);

    // for this, we have to create a mutable value and reference
    // and pass it to the function
    let mut s = String::from("mutable borrow");
    change(&mut s);

    // we also can't create simultaneous mutable references to a mutable value
    // as it can't create something called as data race (a race condition for mutable data)
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    // to prevent data race condition, we can enclose the first reference in a scope
    // so as to enable multiple references while preventing simultaneous ones
    let mut s = String::from("mutable references");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make further `mut` references w/o any problems
    let r2 = &mut s;
    println!("{}", r2);

    // we also can't combine mutable and immutable references simultaneously
    // but we can use immutable references before making a mutable reference of the same value
    // or vice-versa
    let mut s = String::from("combining references");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, {}", r1, r2, r3);

    // rust can't allow dangling references
    let ref_to_nothing = dangle();

    // string slices with an immutable reference to a string
    let s = String::from("string slices");
    let first = &s[0..6];
    let second = &s[7..];
}

fn some_func(some_string: String) {
    println!("{some_string}");
}

fn ref_func(some_string: &String) {
    println!("{some_string}");
}

fn change(some_string: &String) {
    // compiler error
    some_string.push_str("annnnd... changed!");
}

fn change_mut(some_string: &mut String) {
    // works
    some_string.push_str("nope!");
}

// this won't compile as the value of `s` is dropped after function goes out of scope
// and then `&s` references to nothing
fn dangle() -> &String {
    let s = String::from("dangling string");
    &s
}