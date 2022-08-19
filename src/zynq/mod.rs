use clap::{Parser, Subcommand};
use serde::Serialize;
use reqwest::{cookie::Jar, Url};

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
    Book {},
    // See bookable desks
    // List {},
    // Cancel a day's booking
    // Cancel {},
}

pub fn command(zynq: &ZynqCommand) {
    match zynq.command {
        Some(ZynqCommands::Book {}) => {
            book_desk_command();      
        }
        None => {}
    }

}

#[derive(Debug, Serialize)]
struct MultidayBookRequest {
    floorplanID: i32,
    days: Vec<String>,
    preferredSeatID: i32,
    resourceType: String,
    start: String,
    end: String,
    bookingLengthDays: i32,
}

fn book_desk_command() {
    log::debug!("running book desk command");

    let mut days = Vec::new();
    days.push(String::from("2022-09-16"));
    days.push(String::from("2022-09-17"));

    let request_body = MultidayBookRequest {
        floorplanID: 558,
        days: days,
        preferredSeatID: 20606,
        resourceType: String::from("DESK"),
        start: String::from("08:00"),
        end: String::from("17:00"),
        bookingLengthDays: 0,
    };

    log::debug!("Making request to the Zynq API with body: {:?}", request_body);

    let cookie = "sessionid=12345; Domain=zynq.io";
    let url = "https://zynq.io".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);

    let cookie_store = std::sync::Arc::new(jar);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(cookie_store)
        .build().expect("failed to build http client");
    let res = client.post("https://zynq.io/seating/api/book_multiday")
        .json(&request_body)
        .send();

    log::debug!("Got response: {:?}", res.expect("could not unwrap").text());
    
}
