/// Generates a main function which calls the given app init function after including the generated build script code and performing setup.
#[macro_export]
macro_rules! app_main {
    ($func:ident) => {
        // Call the app_init macro to do the include.
        $crate::app_init!();

        fn main() {
            // Do any runtime setup here.
            $crate::start();

            // Call the function indicated by the macro argument.
            $func(BuildCtx);
        }
    };
}

/// Splitting this into its own macro gives users the ability to write their own main function without having to figure out the include if they prefer.
#[macro_export]
macro_rules! app_init {
    () => {
        // This syntax with leading "::" just means we're using the actual core crate regardless of whether the end user might have imported another module named core into the scope this macro is called in.
        ::core::include!(concat!(::core::env!("OUT_DIR"), "/build-ctx.rs"));
    };
}

pub fn start() {
    println!("Starting app")
}
