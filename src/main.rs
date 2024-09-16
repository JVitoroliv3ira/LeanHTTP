mod commands;

fn main() {
    let matches = commands::get_matches();
    match matches.subcommand() {
        Some(("get", _)) => commands::handle_get(),
        Some(("post", _)) => commands::handle_post(),
        _ => println!("Command not found"),
    }
}
