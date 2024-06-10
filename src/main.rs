mod cli;
pub mod crypto;
pub mod utils;

fn main() {
    let interface = cli::Interface;
    interface.run();
}
