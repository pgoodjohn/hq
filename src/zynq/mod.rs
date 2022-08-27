use clap::{Parser, Subcommand};

mod book;
mod configuration;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct ZynqCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<ZynqCommands>,
}

#[derive(Subcommand)]
pub enum ZynqCommands {
    /// Book a desk
    Book {
        #[clap(long)]
        floor: Option<i32>,
        #[clap(short, long)]
        seat: Option<i32>,

        #[clap(short, long, conflicts_with = "from", conflicts_with = "to")]
        date: Option<String>,

        #[clap(short, long, conflicts_with = "date", requires = "to")]
        from: Option<String>,

        #[clap(short, long, conflicts_with = "date", requires = "from")]
        to: Option<String>,
    },
    // See bookable desks
    // List {},
    // Cancel a day's booking
    // Cancel {},
    /// Configurate authentication with the Zynq API
    Auth {
        #[clap(short, long)]
        session_id: String,
    },
}

pub fn command(zynq: &ZynqCommand) {
    match zynq.command.as_ref() {
        Some(ZynqCommands::Book {
            floor,
            seat,
            date,
            from,
            to,
        }) => {
            book::command(
                floor.as_ref(),
                seat.as_ref(),
                date.as_ref(),
                from.as_ref(),
                to.as_ref(),
            )
            .expect("Failed to book a desk 😭");
        }
        Some(ZynqCommands::Auth { session_id }) => {
            authenticate_command(&session_id);
        }
        None => {}
    }
}
fn authenticate_command(session_id: &String) {
    log::debug!("saving session id");
    let config = configuration::Configuration::new(session_id);

    config.save();
}
