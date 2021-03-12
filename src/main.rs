use tuli_lib::Server;

fn main() {
    let server = Server::new(8000, "./public/dub".to_string());

    server.run();

    println!("Hello, world!");
}
