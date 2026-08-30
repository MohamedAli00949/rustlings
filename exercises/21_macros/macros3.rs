// TODO: Fix the compiler error without taking the macro definition out of this
// module.
#[macro_use]
mod macros {
    // OR another method
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
