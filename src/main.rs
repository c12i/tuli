use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use tuli_lib::Server;

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(tuli =>
        (name:crate_name!())
        (about: "An experimental cli for serving static files")
        (long_about:crate_description!())
        (version:crate_version!())
        (author:crate_authors!())
        (@subcommand serve =>
            (about: "Serves static files in a given directory")
            (@arg port: -p --port +takes_value "The port the server should run on, default is 8080")
            (@arg dir: -d --dir +takes_value "The public directory with static files that should be served, defaults to the current directory")
        )
    )
    .get_matches();

    if let Some(sub) = clap.subcommand_matches("serve") {
        match (sub.value_of("port"), sub.value_of("dir")) {
            (None, None) => Server::default().run(),
            (None, Some(dir)) => {
                let server = Server {
                    public_path: dir.to_string(),
                    ..Default::default()
                };
                server.run();
            }
            (Some(port), None) => {
                let server = Server {
                    port: port.parse().unwrap(),
                    ..Default::default()
                };
                server.run();
            }
            (Some(port), Some(dir)) => Server::new(port.parse().unwrap(), dir.to_string()).run(),
        }
    }
    Ok(())
}
