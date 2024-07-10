fn main() {
    assert_eq!(format!("Hello, world!"), "Hello, world!");

    // In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
    assert_eq!(format!("{} days", 31), "31 days");

    // Positional arguments can be used by specifying an integer inside `{}` to specify the position.
    assert_eq!(
        format!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"),
        "Alice, this is Bob. Bob, this is Alice"
    );

    // The internal iterator over the argument has not been advanced by the time the first {} is seen, so it prints the first argument.
    // Then upon reaching the second {}, the iterator has advanced forward to the second argument.
    // Essentially, parameters that explicitly name their argument do not affect parameters that do not name an argument in terms of positional specifiers.
    assert_eq!(format!("{1} {} {0} {}", 1, 2), "2 1 1 2");

    // Also, named arguments can be used.
    assert_eq!(
        format!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        ),
        "the quick brown fox jumps over the lazy dog"
    );

    fn make_string(a: u32, b: &str) -> String {
        format!("{b} {a}")
    }
    assert_eq!(make_string(927, "label"), "label 927");

    // Formatting traits
    //
    // nothing ⇒ Display
    // ? ⇒ Debug
    // x? ⇒ Debug with lower-case hexadecimal integers
    // X? ⇒ Debug with upper-case hexadecimal integers
    // o ⇒ Octal
    // x ⇒ LowerHex
    // X ⇒ UpperHex
    // p ⇒ Pointer in Memory
    // b ⇒ Binary
    // e ⇒ LowerExp
    // E ⇒ UpperExp

    let number = 69420;

    assert_eq!(format!("{}", number), "69420");
    assert_eq!(format!("{:b}", number), "10000111100101100");
    assert_eq!(format!("{:o}", number), "207454");
    assert_eq!(format!("{:x?}", number), "10f2c");
    assert_eq!(format!("{:X?}", number), "10F2C");
    assert_eq!(format!("{:p}", &number).starts_with("0x"), true); // 0x16cf39b4c or something similar
    assert_eq!(format!("{:e}", number), "6.942e4");
    assert_eq!(format!("{:E}", number), "6.942E4");

    // More formatting traits
    //
    // #? - pretty-print the Debug formatting (adds linebreaks and indentation)
    // #x - precedes the argument with a 0x
    // #X - precedes the argument with a 0x
    // #b - precedes the argument with a 0b
    // #o - precedes the argument with a 0o

    assert_eq!(format!("{:#?}", number), "69420");
    assert_eq!(format!("{:#x}", number), "0x10f2c");
    assert_eq!(format!("{:#X}", number), "0x10F2C");
    assert_eq!(format!("{:#b}", number), "0b10000111100101100");
    assert_eq!(format!("{:#o}", number), "0o207454");

    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");

    // Additionally, Rust allows further customization of formatting by using various flags and options within the curly braces {}:
    // Width: {:5} - Specifies a minimum width.
    // Precision: {:.2} - Specifies the number of decimal places.
    // Fill/Align: {:>5} or {:<5} or {:^5} - Aligns the text to the right, left, or center with spaces.
    // Sign: {:+} - Adds a sign for positive numbers.
    // Leading Zeros: {:05} - Pads the number with leading zeros.

    let pi = 3.141592;
    assert_eq!(format!("{:5}", number), "69420");
    assert_eq!(format!("{:.3}", pi), "3.142");
    assert_eq!(format!("{:>10}", number), "     69420");
    assert_eq!(format!("{:<10}", number), "69420     ");
    assert_eq!(format!("{:^10}", number), "  69420   ");
    assert_eq!(format!("{:+}", number), "+69420");
    assert_eq!(format!("{:05}", number), "69420");

    assert_eq!(format!("{num:0>5}", num = 1), "00001");
    assert_eq!(format!("{num:0<5}", num = 1), "10000");
    assert_eq!(format!("{num:05}", num = 1), "00001");

    // You can use named arguments in the format specifier by appending a `$`.
    assert_eq!(format!("{number:0>width$}", number = 1, width = 5), "00001");

    // For Rust 1.58 and above, you can directly capture the argument from a surrounding variable
    let float_number: f64 = 1.0;
    let width: usize = 5;
    assert_eq!(format!("{float_number:>width$}"), "    1");

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond"); // this does not compile
    assert_eq!(
        format!("My name is {0}, {1} {0}", "Bond", "James"),
        "My name is Bond, James Bond"
    );

    println!("Only this will be printed to the console. In Rust, format! macro is used to write formatted text to String.");
}
