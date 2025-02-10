#[macro_export]
macro_rules! service_macro {
    (#simple) => {
        pub fn simple_service_macro() {
            println!("service_macro: simple_service_macro called");
        }
    };
    ($name:ident) => {
        pub fn $name() {
            println!("service_macro: function `{}` called", stringify!($name));
        }
    };
}

service_macro!(example_service_function);