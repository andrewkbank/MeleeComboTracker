//use plotters::prelude::*;
//use plotters_canvas::CanvasBackend;
use std::{
    borrow::BorrowMut, error::Error, fs::File //io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, string
};
use serde::Deserialize;
use svg::{
    node::element::{
        path::Data, Circle, Path
    }, Document, Node
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // There's a good chance these values will remain unused for a while.
struct ComboInfo{
    #[serde(rename = "Start x")]
    start_pos_x:f32,
    #[serde(rename = "Start y")]
    start_pos_y:f32,
    #[serde(rename = "End x")]
    end_pos_x:f32,
    #[serde(rename = "End y")]
    end_pos_y:f32,
    #[serde(rename = "Start Move")]
    start_move:u16,
    #[serde(rename = "End Move")]
    end_move:u16,
    #[serde(rename = "Comboer Character")]
    attacking_char:u16,
    #[serde(rename = "Comboee Character")]
    victim_char:u16,
    #[serde(rename = "Start %")]
    start_percent:f32,
    #[serde(rename = "End %")]
    end_percent:f32,
    #[serde(rename = "Frames Between Moves")]
    between_frames:u16,
    #[serde(rename = "Stage")]
    stage:u16,
}

// Create a web server to host the graphical frontend.
// This is using the single-threaded web server outlined in ch. 20 of The Rust Programming Language
// (doc.rust-lang.org/book)
/*
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
*/

// Convert a given CSV file (created by our program) 
// If we decide to tackle a *massive* volume of replay data all at once, 
// I/O may become a significant bottleneck.
pub fn csv_to_svg(file_path: String) -> Result<(), Box<dyn Error>>{
    // open a CSV file 
    let mut reader = csv::Reader::from_reader(File::open(file_path).unwrap());
    let mut combo_list: Vec<ComboInfo> = Vec::new();
    for result in reader.deserialize(){
        let record: ComboInfo = result?;
        //dbg!(&record);
        combo_list.push(record);
    }
    
    // begin converting the structure to SVG objects
    //
    let max_x = 200;
    let max_y = 100;
    let mut document = Document::new()
        .set("viewBox", (-max_x, -max_y, max_x, max_y));

    for combo in combo_list {
        let data = Data::new()
            // TODO: Translate coordinates to relative units.
            .move_to((combo.start_pos_x, combo.start_pos_y))
            .line_to((combo.end_pos_x, combo.end_pos_y))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

        document.append(path);
    }

    svg::save("test-image.svg", &document).unwrap();
    // write the SVG to file
    Ok(())
}
