fn main() {
    /* immutability */
    let mut _x: i8 = 5;
    _x = 6;
    println!("x is {_x}");
    // mutability is guaranteed when only incoming value is same type
    // _x = 129;
    //println!("x is {_x}");

    /* constants */
    const AKA: i8 = 13;
    const STRING: &'static str = "temporal string";
    const TEMP: i8 = AKA + 1;
    println!("aka is {AKA} string is  {STRING} temp is {TEMP}");

    /* shadowing */
    let x: i8 = 5;
    let x: i8 = x + 1;
    {
        let x: i8 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {spaces}");
    // Throw error because mut variable type is different
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
