mod commands;
mod http_response;

fn main() {
    let matches = commands::get_matches();
    match matches.subcommand() {
        Some(("get", args)) => commands::handle_get(args),
        Some(("post", _)) => commands::handle_post(),
        _ => println!("Command not found"),
    }
}
