/// Exercise 9:
/// Draw the rest of the owl. (Server)
///
/// Implement a game where the server picks a random city name and sends it to
/// all connected clients and let them guess the coordinates.
/// When all clients have answered, send the answer to each of them and then
/// print out the name of client that made the best guess to the console.
///
/// Use this client to test your server.

use std::net::TcpStream;
use apricity::Coordinate;
use rand::prelude::*;
use rust_course_2025::{helpers::exercise_9::city_parser::*, protocol::{ClientMessage, ServerMessage}};

pub enum SocketEvent {
    Connect(u32, TcpStream),
    ServerMessage(u32, ServerMessage),
    ClientMessage(u32, ClientMessage),
    Disconnect(u32),
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cities = load_city_data()?;
    let socket = TcpStream::connect(("127.0.0.1", 12345))?;
    let message_to_server = "Dummy".to_string();
    let client_message = ClientMessage::Hello { name: message_to_server };
    bincode::serialize_into::<&TcpStream, ClientMessage>(&socket, &client_message)?;
    println!("Sending HELLO to server.");


    while let Ok(server_message) = bincode::deserialize_from::<&TcpStream, ServerMessage>(&socket) {
        print!("Server responded: ");
        match server_message {
            ServerMessage::Welcome { server_name } => {
                println!(r#"Welcome sever_name="{server_name}""#);
            },
            ServerMessage::NewRound { city_name } => {
                println!(r#"NewRound city_name="{city_name}]""#);
                let guess = make_guess(&cities)?;
                std::thread::sleep(std::time::Duration::from_millis(2500));
                println!("Guessing {} lat, {} lon", guess.lat(), guess.lon());
                let client_message = ClientMessage::Guess(guess);
                bincode::serialize_into(&socket, &client_message)?;
            }
            ServerMessage::RoundResults { actual_location } => {
                println!("RoundResults actual_location={} lat, {} lon", actual_location.lat(), actual_location.lon());
            }
        }
    }
    Ok(())
}

pub fn make_guess(city_data: &Vec<CityData>) -> Result<Coordinate, Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let new_city_index: usize = rng.gen_range(0..city_data.len());
    let guessed_coordinates = city_data.get(new_city_index).unwrap().coordinates;
    Ok(guessed_coordinates)
}