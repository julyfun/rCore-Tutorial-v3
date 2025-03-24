use std::backtrace::Backtrace;

fn function_c() {
    // Create a new backtrace
    let bt = Backtrace::capture();
    // Print the backtrace
    println!("Call stack trace:\n{:?}", bt);
}

fn function_b() {
    function_c();
}

fn function_a() {
    function_b();
}

fn main() {
    // Enable backtraces with environment variable
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    println!("Printing call stack information:");
    function_a();
}
