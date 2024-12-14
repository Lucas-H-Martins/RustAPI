use infrastructure::configure_env;

// mod middlewares;
mod errors;
mod infrastructure;
fn main() {
    let _ = configure_env();
}
