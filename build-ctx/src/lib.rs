/// Generates a main function which calls the given app init function after including the generated build script code and performing setup.
#[macro_export]
macro_rules! app_main {
    ($func:ident) => {
        ::std::include!(concat!(::std::env!("OUT_DIR"), "/build-ctx.rs"));

        fn main() {
            // Do any runtime setup here.

            // Call the function indicated by the macro argument.
            $func(BuildCtx);
        }
    };
}
