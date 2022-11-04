build_ctx::app_main!(app_init);

/// This is effectively our main function, however `build_ctx::app_main` generates the actual main function to allow the crate to do extra runtime setup if necessary.
fn app_init(ctx: BuildCtx) {
    println!("{} v{}", ctx.name(), ctx.version());
}
