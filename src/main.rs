extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

mod models;

use clap::{Arg, App, SubCommand};
use hyper::Url;
use hyper::client::Client;
use hyper::status::StatusCode;
use hyper::client::response::Response;
use models::{LabsReport, LabsEndpoint};
use rustc_serialize::json;
use std::{thread, time};
use std::io::Read;

lazy_static! {
    static ref API_ENDPOINT: String = "https://api.ssllabs.com/api/v2/".to_owned();
    static ref CLIENT: Client = Client::new();
}

fn make_get_request(url: String) -> (String, Response) {
    let finalized_url = Url::parse(&url);

    if finalized_url.is_err() {
        panic!("Somehow got an invalid URL: [ {} ]", url);
    }

    let finalized_url = finalized_url.unwrap();

    let resp = CLIENT.get(finalized_url).send();

    if resp.is_err() {
        panic!("Failed to send HTTP Request to: [ {} ] cause: [ {:?} ]", url, resp.err().unwrap());
    }

    let mut resp = resp.unwrap();

    let mut return_value = String::new();
    resp.read_to_string(&mut return_value).unwrap();

    (return_value, resp)
}

fn ensure_success(resp: Response) {
    if resp.status == StatusCode::BadRequest {
        panic!("Somehow invalid parameters were passed.");
    } else if resp.status == StatusCode::TooManyRequests {
        panic!("You're sending too many requests! Slow down cowboy!");
    } else if resp.status == StatusCode::InternalServerError {
        panic!("Internal Server Error :(");
    } else if resp.status == StatusCode::ServiceUnavailable {
        panic!("Service not available right now!");
    }
}

fn start_new_scan(host: String) {
    info!("Starting Brand New Scan for: [ {} ]", host);
    let (_, _) = make_get_request(format!("{}analyze?host={}&startNew=true&all=done", *API_ENDPOINT, host));
    while true {
        // Be nice to SSLLabs
        thread::sleep(time::Duration::from_secs(5));

        let (first_req, resp) = make_get_request(format!("{}analyze?host={}", *API_ENDPOINT, host));
        ensure_success(resp);
        let decoded: LabsReport = json::decode(&first_req).unwrap();

        info!("Status: [ {} ]", decoded.status);
        if decoded.status == "READY".to_string() {
            let endpoints = decoded.endpoints;

            for ref endpoint in endpoints.iter() {
                let (endpoint_req, end_resp) = make_get_request(format!("{}getEndpointData?host={}&s={}", *API_ENDPOINT, host, endpoint.ipAddress));
                ensure_success(end_resp);
                let results: LabsEndpoint = json::decode(&endpoint_req).unwrap();
                debug!("{:?}", results);
                println!("IP Address: [ {} ]. Grade: [ {} ].", results.ipAddress, results.grade.unwrap());
            }
            return;
        }
    }
}

fn get_endpoint_data(host: String, ip: String) {
    info!("Grabbing Endpoint Data for: [ {} ]", host);
    let (string_data, resp) = make_get_request(format!("{}getEndpointData?host={}&s={}", *API_ENDPOINT, host, ip));
    ensure_success(resp);
    let results: Result<LabsEndpoint, _> = json::decode(&string_data);

    if results.is_err() {
        panic!("Couldn't grab old JSON: [ {:?} ]", results.err().unwrap());
    }

    let results: LabsEndpoint = results.unwrap();

    debug!("{:?}", results);
    println!("IP Address: [ {} ]. Grade: [ {} ].", results.ipAddress, results.grade.unwrap());
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting SslScan...");

    let matches = App::new("sslscan")
        .version("1.0.0")
        .about("Scans SSL Certs with SSL Labs so you don't have too.")
        .subcommand(
            SubCommand::with_name("scan_new")
                .version("1.0.0")
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .help("The host to scan.")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("get_old")
                .version("1.0.0")
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .help("The host to scan.")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("ip")
                        .short("i")
                        .help("The IP of the host.")
                        .required(true)
                        .takes_value(true)
                )
        )
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("scan_new") {
        return start_new_scan(matches.value_of("host").unwrap().to_owned());
    }

    if let Some(matches) = matches.subcommand_matches("get_old") {
        return get_endpoint_data(matches.value_of("host").unwrap().to_owned(), matches.value_of("ip").unwrap().to_owned());
    }

    println!("Please provide A valid command! Use `--help` if you're unsure.");
}
