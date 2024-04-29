//2.Primitive Data Types - BEGIN

fn declare_variables() {
    println!("\ndeclare_variables() {{\n");

    let immutable_x = 10;
    println!("immutable_x is {}", immutable_x);

    let mut mutable_x = 100;
    println!("Before mutable_x is {}", mutable_x);

    mutable_x += 1;
    println!("After mutable_x is {}", mutable_x);

    println!("\n}}\n");
}

fn integer_datatypes() {
    println!("\ninteger_datatypes() {{\n");

    // Declaring different types of integer variables
    let x: i8 = 10;
    let y: i16 = 1000;
    let z: i32 = 10000;
    let w: i64 = 100000;

    // Printing the values
    println!("Value of x (i8): {}", x);
    println!("Value of y (i16): {}", y);
    println!("Value of z (i32): {}", z);
    println!("Value of w (i64): {}", w);

    println!("\n}}\n");
}

fn float_datatypes() {
    println!("\nfloat_datatypes() {{\n");

    // Declaring different types of floating-point variables
    let x: f32 = 3.14;
    let y: f64 = 6.28;

    // Printing the values
    println!("Value of x (f32): {}", x);
    println!("Value of y (f64): {}", y);

    println!("\n}}\n");
}

fn arithmetic_operations() {
    println!("\narithmetic_operations() {{\n");

    // Integer types
    let int_a: i32 = 10;
    let int_b: i32 = 20;

    // Floating-point types
    let float_a: f32 = 3.14;
    let float_b: f32 = 6.28;

    // Addition
    println!("Addition:");
    println!("int_a + int_b = {}", int_a + int_b);
    println!("float_a + float_b = {}", float_a + float_b);

    // Subtraction
    println!("Subtraction:");
    println!("int_a - int_b = {}", int_a - int_b);
    println!("float_b - float_a = {}", float_b - float_a);

    // Multiplication
    println!("Multiplication:");
    println!("int_a * int_b = {}", int_a * int_b);
    println!("float_a * float_b = {}", float_a * float_b);

    // Division
    println!("Division:");
    println!("int_b / int_a = {}", int_b / int_a);
    println!("float_b / float_a = {}", float_b / float_a);

    // Modulus (only for integer types)
    println!("Modulus:");
    println!("int_a % int_b = {}", int_a % int_b);

    println!("\n}}\n");
}

fn print_format_examples() {
    println!("\nprint_format_examples() {{\n");

    // Variables for demonstration
    let name = "Alice";
    let age = 30;
    let height = 1.75;

    // Basic formatting
    println!(
        "Hello, {}! You are {} years old and {} meters tall.",
        name, age, height
    );

    // Width and alignment
    println!("Name: {:<10} Age: {:>5} Height: {:.2}", name, age, height);

    // Number formatting
    let num = 123456789;
    println!("Number: {}", num);
    println!("Number (comma separated): {:#?}", num);

    // Hexadecimal, octal, and binary
    let num_hex = 255;
    let num_octal = 64;
    let num_binary = 10;
    println!(
        "Hex: {:x}, Octal: {:o}, Binary: {:b}",
        num_hex, num_octal, num_binary
    );

    // Displaying special characters
    println!("Special characters: {} {} {}", '♥', '\u{2764}', '\u{1F496}');

    // Scientific notation
    let scientific_num = 1234.5678;
    println!("Scientific notation: {:.2e}", scientific_num);

    // Padding with zeroes
    let padded_num = 42;
    println!("Padded with zeroes: {:05}", padded_num);

    // Debugging output
    let debug_value = "Hello, world!";
    println!("Debug output: {:?}", debug_value);

    // Custom formatting with a custom type
    struct MyType {
        value: i32,
    }
    impl std::fmt::Display for MyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyType: {}", self.value)
        }
    }
    let custom_value = MyType { value: 42 };
    println!("Custom formatted value: {}", custom_value);

    println!("\n}}\n");
}

fn bitwise_operations(a: u8, b: u8) {
    println!("\nbitwise_operations() {{\n");

    println!("a: {:08b}", a);
    println!("b: {:08b}", b);

    // Bitwise AND
    let and_result = a & b;
    println!("a & b: {:08b}", and_result);

    // Bitwise OR
    let or_result = a | b;
    println!("a | b: {:08b}", or_result);

    // Bitwise XOR
    let xor_result = a ^ b;
    println!("a ^ b: {:08b}", xor_result);

    // Bitwise NOT
    let not_result_a = !a;
    let not_result_b = !b;
    println!("!a: {:08b}", not_result_a);
    println!("!b: {:08b}", not_result_b);

    // Bitwise left shift
    let left_shift_result_a = a << 2;
    let left_shift_result_b = b << 3;
    println!("a << 2: {:08b}", left_shift_result_a);
    println!("b << 3: {:08b}", left_shift_result_b);

    // Bitwise right shift
    let right_shift_result_a = a >> 1;
    let right_shift_result_b = b >> 2;
    println!("a >> 1: {:08b}", right_shift_result_a);
    println!("b >> 2: {:08b}", right_shift_result_b);

    println!("\n}}\n");
}

fn boolean_operations() {
    println!("\nboolean_operations() {{\n");

    // Your method logic goes here
    let a = true;
    let b = false;

    println!("a: {}", a);
    println!("b: {}", b);

    // Logical AND
    let and_result = a && b;
    println!("a && b: {}", and_result);

    // Logical OR
    let or_result = a || b;
    println!("a || b: {}", or_result);

    // Logical NOT
    let not_result_a = !a;
    let not_result_b = !b;
    println!("!a: {}", not_result_a);
    println!("!b: {}", not_result_b);

    // Conditional operators
    let conditional_result = if a { "a is true" } else { "a is false" };
    println!("Conditional Result: {}", conditional_result);

    println!("\n}}\n");
}

fn comparison_operations(a: i32, b: i32, c: f64, d: f64) {
    println!("\ncomparison_operations() {{\n");

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    // Greater than
    println!("a > b: {}", a > b);
    println!("c > d: {}", c > d);

    // Less than
    println!("a < b: {}", a < b);
    println!("c < d: {}", c < d);

    // Greater than or equal to
    println!("a >= b: {}", a >= b);
    println!("c >= d: {}", c >= d);

    // Less than or equal to
    println!("a <= b: {}", a <= b);
    println!("c <= d: {}", c <= d);

    // Equal to
    println!("a == b: {}", a == b);
    println!("c == d: {}", c == d);

    // Not equal to
    println!("a != b: {}", a != b);
    println!("c != d: {}", c != d);

    println!("\n}}\n");
}

//2.Primitive Data Types - END

fn main() {
    declare_variables();

    integer_datatypes();

    float_datatypes();

    arithmetic_operations();

    print_format_examples();

    let a: u8 = 0b1100_1010;
    let b: u8 = 0b1010_1010;

    bitwise_operations(a, b);

    boolean_operations();

    let a: i32 = 5;
    let b: i32 = 10;
    let c: f64 = 3.14;
    let d: f64 = 6.28;

    comparison_operations(a, b, c, d);
}
