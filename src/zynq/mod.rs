use clap::{Parser, Subcommand};

mod book;
mod config;
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
    /// Configure the Zynq command
    Config(config::ConfigCommand),
}

pub fn command(zynq: &ZynqCommand) {
    match zynq.command.as_ref() {
        Some(ZynqCommands::Book {
            seat,
            date,
            from,
            to,
        }) => match book::command(seat.as_ref(), date.as_ref(), from.as_ref(), to.as_ref()) {
            Ok(r) => match r.days {
                Some(days) => {
                    for day in days.into_iter() {
                        log::info!("Booked desk for {}", day);
                        return;
                    }
                }
                None => {
                    log::info!("Booking request was successful, but no new days were booked.");
                    log::info!("Do you already have a desk booked? Zynq doesn't tell me 😢");
                }
            },
            Err(e) => {
                log::error!("{}", e);
            }
        },
        Some(ZynqCommands::Config(command)) => {
            config::command(command);
        }
        None => {}
    }
}
