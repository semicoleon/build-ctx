use user::print_info;

// Includes the generated code for us, without generating a main function.
build_ctx::app_init!();

fn main() {
    build_ctx::start();

    println!("app_init");
    let ctx = BuildCtx;
    print_info(ctx.name(), ctx.version());
}
