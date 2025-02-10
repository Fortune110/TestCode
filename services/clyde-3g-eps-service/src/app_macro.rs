#[macro_export]
macro_rules! app_macro {
    (#simple) => {
        pub fn simple_app_macro() {
            println!("app_macro: simple_app_macro called");
        }
    };
    ($name:ident) => {
        pub fn $name() {
            println!("app_macro: function `{}` called", stringify!($name));
        }
    };
}


app_macro!(example_app_function);