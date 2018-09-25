mod command;
mod shell;
mod net;
mod matrix_api;

fn main() {
    let bot_shell = shell::Shell::new();
    let net=net::Net::Matrix(matrix_api::matrix_api::new());
}
