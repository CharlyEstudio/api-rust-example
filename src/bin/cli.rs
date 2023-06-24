use basquet::commands::users::{create_user, list_user, delete_user};
use clap::{Command, Arg};

extern crate basquet;

fn main() {
  let matches =Command::new("basquet")
    .about("Basquet commands")
    .arg_required_else_help(true)
    .subcommand(
      Command::new("users")
        .about("Users commands")
        .arg_required_else_help(true)
        .subcommand(
          Command::new("create")
            .about("Create a user with multiple roles attached")
            .arg_required_else_help(true)
            .arg(Arg::new("username").required(true))
            .arg(Arg::new("password").required(true))
            .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
        )
        .subcommand(
          Command::new("list")
          .about("List all available users")
        )
        .subcommand(
          Command::new("delete")
          .about("Delete user by ID")
          .arg(Arg::new("id").required(true).value_parser(clap::value_parser!(i32)))
        )
    ).get_matches();

  match matches.subcommand() {
    Some(("users", matchs_users)) => match matchs_users.subcommand() {
      Some(("create", matches_create)) => create_user(
        matches_create.get_one::<String>("username").unwrap().to_owned(),
        matches_create.get_one::<String>("password").unwrap().to_owned(),
        matches_create.get_many::<String>("roles").unwrap().map(|v| v.to_string()).collect()
      ),
      Some(("list", _)) => list_user(),
      Some(("delete", matches_delete)) => delete_user(
        matches_delete.get_one::<i32>("id").unwrap().to_owned()
      ),
      _ => println!("No subcommands found for users")
    },
    _ => println!("No subcommands found"),
  }
}
