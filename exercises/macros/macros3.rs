// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
        ( $x:expr ) => {
            println!("This macro has one argument: {}", $x);
        };
    }
}

fn main() {
    my_macro!();
    my_macro!(17);
}
