use user::print_info;

// Generates the main function and includes the generated code for us.
build_ctx::app_main!(app);

/// This is effectively our main function, however `build_ctx::app_main` generates the actual main function to allow the crate to do extra runtime setup if necessary.
fn app(ctx: BuildCtx) {
    println!("app_main");
    print_info(ctx.name(), ctx.version())
}
