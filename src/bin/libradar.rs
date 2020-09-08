use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("libradar [command]")
        .action(default_action)
        .flag(
            Flag::new("version", FlagType::Bool)
                .usage("libradar --version|-V")
                .alias("V"),
        )
        .flag(
            Flag::new("help", FlagType::Bool)
                .usage("libradar --help|-h")
                .alias("h"),
        )
        .command(help_command())
        .command(version_command())
        .command(detect_command())
        .command(insert_command())
        .command(remove_command())
        .command(import_command())
        .command(export_command());

    app.run(args)
}

fn default_action(c: &Context) {
    if c.bool_flag("help") {
        help_action(c);
    } else if c.bool_flag("version") {
        version_action(c);
    } else {
        c.help();
    }
}

fn help_action(c: &Context) {
    c.help();
}

fn version_action(_c: &Context) {
    println!("libradar version {}", env!("CARGO_PKG_VERSION"));
}

fn detect_action(_c: &Context) {
    todo!();
}

fn insert_action(_c: &Context) {
    todo!();
}

fn remove_action(_c: &Context) {
    todo!();
}

fn import_action(_c: &Context) {
    todo!();
}

fn export_action(_c: &Context) {
    todo!();
}

fn help_command() -> Command {
    Command::new("help")
        .usage("libradar help")
        .action(help_action)
}

fn version_command() -> Command {
    Command::new("version")
        .usage("libradar version")
        .action(version_action)
}

fn detect_command() -> Command {
    Command::new("detect")
        .usage("libradar detect <apks>")
        .action(detect_action)
}

fn remove_command() -> Command {
    Command::new("remove")
        .usage("libradar remove")
        .action(remove_action)
}

fn insert_command() -> Command {
    Command::new("insert")
        .usage("libradar insert")
        .action(insert_action)
}

fn import_command() -> Command {
    Command::new("import")
        .usage("libradar import")
        .action(import_action)
}

fn export_command() -> Command {
    Command::new("export")
        .usage("libradar export")
        .action(export_action)
}
