use cgmath::Vector2; 
use csv;
use plotters::prelude::*;
//use plotters_canvas::CanvasBackend;
use std::{
    error::Error, fs::File, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, string
};

struct ComboInfo{
    start_pos:Vector2<f32>,
    end_pos:Vector2<f32>,
    start_move:u16,
    end_move:u16,
    attacking_char:u16,
    victim_char:u16,
    start_percent:f32,
    end_percent:f32,
    between_frames:u16,
    stage:u16,
}

// Create a web server to host the graphical frontend.
// This is using the single-threaded web server outlined in ch. 20 of The Rust Programming Language
// (doc.rust-lang.org/book)
pub fn server_init(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //println!("SERVER: Connection to listener established.")
        handle_connection(stream);
    }
    Ok(())
}

// Respond to an incoming TCP stream.
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    //println!("Request: {:#?}", http_request);
    stream.write_all(response.as_bytes()).unwrap();
}

/*
fn render_init() -> Result<(), Box<dyn std::error::Error>> {
    // Set backend to SVG, start drawing.
    let mut backend = SVGBackend::new("output.svg", (800, 600));
    //let mut backend = CanvasBackend::new();
    Ok(())
}
*/

// Convert a given CSV file (created by our analyzer) 
pub fn csv_to_svg(path: String) -> Result<(), Box<dyn Error>>{
    // open a CSV file 
    let mut reader = csv::Reader::from_reader(File::open(path).unwrap());
    for result in reader.records(){
        let record = result?;
        dbg!(record);
    }
    // bring all of the data into a new structure
    // close the file 
    // begin converting the structure to SVG objects
    // write the SVG to file
    Ok(())
}
