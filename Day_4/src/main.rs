fn main() {
    let s1 = String::from("hello");

    let s2 = s1;

    // println!("value of s2 is {}", s1); // wont work as we've trasnfered the ownership from s1 to s2
    println!("value of s2 is {}", s2);

    let x: i32 = 5;
    let y = String::from("patika");
    let z = y; // now z has the ownership of y
    println!("value of x is {}, and value of z is {}", x, z);
}
