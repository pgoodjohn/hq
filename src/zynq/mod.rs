use clap::{Parser, Subcommand};
use serde::Serialize;
use reqwest::{cookie::Jar, Url};
use chrono::prelude::*;
extern crate dirs;

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

        #[clap(short,long)]
        seat: Option<i32>,

        #[clap(short,long)]
        date: Option<String>,

        #[clap(short,long)]
        from: Option<String>,

        #[clap(short,long)]
        to: Option<String>
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
            to
        }) => {
            book_desk_command(
                floor.as_ref(),
                seat.as_ref(),
                date.as_ref(),
                from.as_ref(),
                to.as_ref()
            );      
        }
        Some(ZynqCommands::Auth { session_id }) => {
            authenticate_command(&session_id);
        }
        None => {}
    }

}

#[derive(Debug, Serialize)]
struct MultidayBookRequest {
    #[serde(rename="floorplanID")]
    floorplan_id: i32,
    days: Vec<String>,
    #[serde(rename="preferredSeatID")]
    preferred_seat_id: i32,
    #[serde(rename="resourceType")]
    resource_type: String,
    start: String,
    end: String,
    #[serde(rename="bookingLengthDays")]
    booking_length_days: i32,
}

struct MultiDayBookRequestBuilder<'a> {
    floor_id: &'a i32,
    days: Vec<String>,
    seat_id: &'a i32,
}

impl<'a> MultiDayBookRequestBuilder<'a> { 

    fn new() -> MultiDayBookRequestBuilder<'a> {
        let empty_days_vec = Vec::<String>::new();
        MultiDayBookRequestBuilder{
            floor_id: &558, // Default here should load from config
            days: empty_days_vec,
            seat_id: &20606, // default here should load from config
        }
    }

    fn floor(mut self, floor_id: Option<&'a i32>) -> Self {
        match floor_id {
            Some(floor) => {
                self.floor_id = floor;
            },
            None => {},
        }

        self
    }

    fn seat(mut self, seat_id: Option<&'a i32>) -> Self {
        match seat_id {
            Some(seat) => {
                self.seat_id = seat;
            },
            None => {},
        }
        
        self
    }

    fn date(mut self, date: Option<&String>) -> Self {
        assert_eq!(0, self.days.len());

        // TODO: Validate dates!!

        match date {
            Some(date) => {
                self.days.push(String::from(date));
            },
            None => {},
        }

        self
    }

    fn period(mut self, from: Option<&'a String>, to: Option<&'a String>) -> Self {
        if self.days.len() > 0 {
            // TODO: Better error handling for this
            panic!("Can't handle date and period together!");
        }

        let from_date_string: &'a String;
        let to_date_string: &'a String;

        match from {
            Some(date) => {
                from_date_string = date; 
            },
            None => {
                return self;
            }
        }

        match to {
            Some(date) => {
                to_date_string = date;
            },
            None => {
                return self;
            }
        }

        let from_date_naive = NaiveDate::parse_from_str(from_date_string, "%Y-%m-%d").expect("Failed to parse from date");
        let to_date_naive = NaiveDate::parse_from_str(to_date_string, "%Y-%m-%d").expect("Failed to parse from date");
       
        if to_date_naive.num_days_from_ce() < from_date_naive.num_days_from_ce() {
            panic!("To date needs to be after from date");
        }

        let dates_to_book = determine_period(from_date_naive, to_date_naive);

       
        self.set_dates(dates_to_book);

        log::debug!("Prepared dates to book via period {:?}", self.days);

        self
    }

    fn set_dates(&mut self, dates_vec: Vec<String>) -> &Self {

        log::debug!("Setting dates for request to: {:?}", dates_vec);

        dates_vec.iter().for_each(|date| self.days.push(date.to_string()));

        log::debug!("Set dates for request to: {:?}", self.days);

        self
    }

    fn spawn(self) -> MultidayBookRequest {
        MultidayBookRequest {
            floorplan_id: self.floor_id.clone(),
            days: self.days.clone(),
            preferred_seat_id: self.seat_id.clone(),
            resource_type: String::from("DESK"),
            start: String::from("08:00"),
            end: String::from("17:00"),
            booking_length_days: 0,
        }
    }
}

fn determine_period(from: NaiveDate, to: NaiveDate) -> Vec<String> {

    let date_difference = to - from;

        let dates_to_book: Vec<String> = from.iter_days().take(date_difference.num_days().try_into().expect("Something terrible happened")).map(
                move |date| {
                    log::info!("{}", date);
                match date.weekday() {
                    Weekday::Sat => {
                        log::info!("{} is a Saturday, skipping it.", "%Y-%m-%d");
                        let no_date: String = String::from("");
                        return no_date;
                    },
                    Weekday::Sun => {
                        log::info!("{} is a Sunday, skipping it.", "%Y-%m-%d");
                        let no_date: String = String::from("");
                        return no_date;
                    }
                    _ => {
                        return format_date(date);
                    }
            }
        }).filter(|x| x.eq(&"") == false).collect();

        log::debug!("Prepared dates to book: {:?}", dates_to_book);

    return dates_to_book;
}

fn format_date(date: NaiveDate) -> String {
        let string_date = date.format("%Y-%m-%d").to_string();

        return string_date;
    }

fn book_desk_command(
       floor: Option<&i32>,
       seat: Option<&i32>,
       date: Option<&String>,
       from: Option<&String>,
       to: Option<&String>
    ){
    log::debug!("running book desk command");

    let mut days = Vec::new();
    days.push(String::from("2022-09-16"));
    days.push(String::from("2022-09-17"));

    let request = MultiDayBookRequestBuilder::new()
        .seat(seat)
        .floor(floor)
        .date(date)
        .period(from, to)
        .spawn();


    log::debug!("Making request to the Zynq API with body: {:?}", request);

    let config = configuration::Configuration::load();
    let cookie = format!("sessionid={}; Domain=zynq.io", config.session_id);
    let url = "https://zynq.io".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let cookie_store = std::sync::Arc::new(jar);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(cookie_store)
        .build().expect("failed to build http client");
    let res = client.post("https://zynq.io/seating/api/book_multiday")
        .json(&request)
        .send();

    log::debug!("Got response: {:?}", res.expect("could not unwrap").text());
    
}

fn authenticate_command(session_id: &String) {
   log::debug!("saving session id");
   let config = configuration::Configuration::new(session_id);

   config.save();
}

