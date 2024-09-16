use clap::Command;

pub fn get_matches() -> clap::ArgMatches {
    Command::new("lean_http")
        .version("v0.1.0")
        .author("Joao Vitor de Oliveira Santos")
        .about("A lightweight HTTP client for making quick HTTP requests")
        .subcommand(
            Command::new("get")
                .about("Makes an HTTP GET request to the specified URL")
                .arg(
                    clap::Arg::new("url")
                        .help("The URL to send the GET request to")
                        .required(true)
                        .value_name("URL")
                )
                .arg(
                    clap::Arg::new("header")
                        .long("header")
                        .help("The header to include in the GET request")
                        .value_name("HEADER")
                        .action(clap::ArgAction::Append)
                )
        )
        .subcommand(
            Command::new("post")
                .about("Makes an HTTP POST request to the specified URL with optional data")
                .arg(
                    clap::Arg::new("url")
                        .help("The URL to send the POST request to")
                        .required(true)
                        .value_name("URL")
                )
                .arg(
                    clap::Arg::new("data")
                        .short('d')
                        .long("data")
                        .help("The data to include in the POST request")
                        .value_name("DATA")
                        .action(clap::ArgAction::Append)
                )
                .arg(
                    clap::Arg::new("header")
                        .long("header")
                        .help("The header to include in the POST request")
                        .value_name("HEADER")
                        .action(clap::ArgAction::Append)
                )
        )
        .get_matches()
}