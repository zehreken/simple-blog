mod blog;
mod server;
mod utils;
mod zettelkasten;

fn main() {
    zettelkasten::run();

    blog::run();

    server::start("http://127.0.0.1:4000/index.html");
}
