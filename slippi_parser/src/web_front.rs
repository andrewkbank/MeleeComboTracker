use serde::Deserialize;
use std::{
    borrow::{Borrow, BorrowMut}, collections::HashMap, default, error::Error, fs::File //io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, string
};
use svg::{
    node::element::{path::Data, Circle, Path, Polyline},
    Document, Node,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // There's a good chance these values will remain unused for a while.
struct ComboInfo {
    #[serde(rename = "Start x")]
    start_pos_x: f32,
    #[serde(rename = "Start y")]
    start_pos_y: f32,
    #[serde(rename = "End x")]
    end_pos_x: f32,
    #[serde(rename = "End y")]
    end_pos_y: f32,
    #[serde(rename = "Start Move")]
    start_move: u16,
    #[serde(rename = "End Move")]
    end_move: u16,
    #[serde(rename = "Comboer Character")]
    attacking_char: u16,
    #[serde(rename = "Comboee Character")]
    victim_char: u16,
    #[serde(rename = "Start %")]
    start_percent: f32,
    #[serde(rename = "End %")]
    end_percent: f32,
    #[serde(rename = "Frames Between Moves")]
    between_frames: u16,
    #[serde(rename = "Stage")]
    stage: u16,
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
*/

// stage boundaries: slippilab

struct StagePathInfo {
    // Main stage geometry as polyline followed by platforms as lines.
    // Have to keep the raw values like this so we can adjust for SVG size.
    polyline:[(f32, f32)],
    platforms:[[(f32, f32)]]
}

mod stage_path_constants {
    use super::StagePathInfo;

    const BATTLEFIELD:StagePathInfo = StagePathInfo {
        polyline:[(-68.4, 0), 
                (-68.4, 0),
                (65.0, -6.0),
                (36.0, -19.0),
                (39.0, -21.0),
                (33.0, -25.0),
                (30.0, -29.0),
                (29.0, -35.0),
                (10.0, -40.0),
                (10.0, -30.0),
                (-10.0, -30.0),
                (-10.0, -40.0),
                (-29.0, -35.0),
                (-30.0, -29.0),
                (-33.0, -25.0),
                (-39.0, -21.0),
                (-36.0, -19.0),
                (-65.0, -6.0),
                (-68.4, 0.0)],
        platforms: [[(-57.6, 27.2), (-20.0, 27.2)],
            [(20.0, 27.2), (56.6, 27.2)],
            [(-18.8, 54.4), (18.8, 54.4)]]
    };

    const DREAMLAND:StagePathInfo = StagePathInfo{
        polyline:[(-76.5, -11.0),
                    (-77.25, 0.0),
                    (77.25, 0.0),
                    (76.5, -11.0),
                    (76.5, -11.0),
                    (65.75, -36.0)
                    (-65.75, -36.0),
                    (-76.5, -11.0)],
        platforms:[[(-61.393, 30.142), (-31.725, 30.142)],
                   [(31.704, 30.243), (63.075, 30.243)],
                   [(-19.018, 51.425), (19.017, 51.425)]]
    };

    const FINAL_DESTINATION:StagePathInfo = StagePathInfo {
        polyline:[(-85.6, 0.0),
                    (85.6, 0.0),
                    (85.6, -10.0)
                    (65.0, -20.0),
                    (65.0, -30.0)
                    (60.0, -47.0)
                    (50.0, -55.0)
                    (45.0, -56.0),
                    (-45.0, -56.0)
                    (-50.0, -55.0)
                    (-60.0, -47.0)
                    (-65.0, -30.0)
                    (-65.0, -20.0)
                    (-85.6, -10.0)
                    (-85.6, 0.0)],
        platforms: [],
    };

    const YOSHIS_STORY:StagePathInfo = StagePathInfo {
        polyline:[(-54.0, -91.0),
                    (-54.0, -47.0),
                    (-53.0, -46.0),
                    (-53.0, -31.0),
                    (-54.0, -30.0),
                    (-54.0, -28.0),
                    (-53.0, -12.0),
                    (-53.0, -12.0),
                    (-54.0, -11.0),
                    (-55.0, -8.0),
                    (-56.0, -7.0),
                    (-56.0, -3.5),
                    (-39.0, 0.0),
                    (39.0, 0.0),
                    (56.0, -3.5),
                    (56.0, -7.0),
                    (55.0, -8.0),
                    (54.0, -11.0),
                    (53.0, -12.0),
                    (53.0, -27.0),
                    (54.0, -28.0),
                    (54.0, -30.0),
                    (53.0, -31.0),
                    (53.0, -46.0),
                    (54.0, -47.0),
                    (54.0, -91.0),
                    (-54.0, -91.0)],
        platforms:[[(-59.5, 23.45), (-28.0, 23.45)],
            [(28.0, 23.45), (59.5, 23.45)], 
            [(-15.75, 42), (15.75, 42)]]
    };

    const FOUNTAIN_OF_DREAMS:StagePathInfo = StagePathInfo {
        polyline:[(-63.33, 0.62),
                    (-53.5, 0.62),
                    (-51.0, 0.0),
                    (-51.0, 0.0),
                    (53.5, 0.62),
                    (63.33, 0.62),
                    (63.35, 0.62),
                    (63.35, -4.5),
                    (59.33, -15.0),
                    (56.9, -19.5),
                    (55.0, -27.0),
                    (52.0, -32.0),
                    (48.0, -38.0),
                    (41.0, -42.0),
                    (19.0, -49.5),
                    (13.0, -54.5),
                    (10.0, -62.0),
                    (8.8, -72.0),
                    (8.8, -150.0),
                    (-8.8, -150.),
                    (-8.8, -72.0),
                    (-10.0, -62.0),
                    (-13.0, -54.5),
                    (-19.0, -49.5),
                    (-41.0, -42.0),
                    (-48.0, -38.0),
                    (-52.0, -32.0),
                    (-55.0, -27.0),
                    (-56.9, -19.5),
                    (-59.33, -15.0),
                    (-63.35, -4.5),
                    (-63.35, 0.62),
                    (-63.35, -4.5),
                    (-63.35, -0.62)],
        platforms:[[(-49.5, 16.125), (-21.0, 16.125)],
                    [(21.0, 22.125), (49.5, 22.125)],
                    [(-14.25, 42.75), (14.25, 42.75)]]
    };

    const POKEMON_STADIUM:StagePathInfo = StagePathInfo {
        polyline:[(87.75, 0.0),
                    (87.75, -4.0),
                    (73.75, -15.0),
                    (73.75, -17.75),
                    (60.0, -17.75),
                    (60.0, -38.0),
                    (15.0, -60.0),
                    (15.0, -112.0),
                    (-15.0, -112.0),
                    (-15.0, -60.0),
                    (-60.0, -38.0),
                    (-60.0, -17.75),
                    (-73.75, -17.75),
                    (-73.75, -15.0),
                    (-87.75, -4.0),
                    (-87.75, 0.0),
                    (87.75, 0.0)],
        platforms:[[(-55.0, 25.0), (-25.0, 25.0)],
                    [(25.0, 25.0), (55.0, 25.0)]]
    };
}

// Convert a given CSV file (created by our program)
// If we decide to tackle a *massive* volume of replay data all at once,
// I/O may become a significant bottleneck.
pub fn csv_to_svg(file_path: String) -> Result<(), Box<dyn Error>> {
    // open a CSV file
    let mut reader = csv::Reader::from_reader(File::open(file_path).unwrap());
    let mut combo_list: Vec<ComboInfo> = Vec::new();
    for result in reader.deserialize() {
        let record: ComboInfo = result?;
        //dbg!(&record);
        combo_list.push(record);
    }

    // Begin converting the structure to SVG objects.
    let max_x = 400.0;
    let max_y = 200.0;

    let mut stage_documents: HashMap<u16, Document> = HashMap::new();

    for combo in 0..combo_list.len() {
        // separate combos by stage.
        if !stage_documents.contains_key(&combo_list[combo].stage) {
            let document = Document::new()
                .set("width", "100%")
                .set("preserveAspectRatio", "xMidYMid meet")
                .set("viewBox", (0, 0, max_x, max_y));

            stage_documents.insert(combo_list[combo].stage, document);
        }

        // For each combo, add a new line from start to end.
        let data = Data::new()
            // TODO: Translate coordinates to relative units.
            .move_to((
                combo_list[combo].start_pos_x + (max_x / 2.0) as f32,
                max_y - (combo_list[combo].start_pos_y + (max_y / 2.0) as f32),
            ))
            .line_to((
                combo_list[combo].end_pos_x + (max_x / 2.0) as f32,
                max_y - (combo_list[combo].end_pos_y + (max_y / 2.0) as f32),
            ))
            .close();

        // TODO: add arrow glyph at end of line.

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("opacity", 0.1)
            .set("stroke-width", 1)
            .set("d", data)
            .set("id", combo);

        stage_documents
            .get_mut(&combo_list[combo].stage)
            .unwrap()
            .append(path);
    }

    // Add stage geometry and write the SVG to file.
    for (stage, doc) in stage_documents {
        let stage_paths = Group::new().set("stroke", "blue");

        let mut stage_path_const:StagePathInfo = StagePathInfo::new();
        match stage {
            2 => stage_path_const = stage_path_constants::FOUNTAIN_OF_DREAMS,
            3 => stage_path_const = stage_path_constants::POKEMON_STADIUM,
            8 => stage_path_const = stage_path_constants::YOSHIS_STORY,
            28 => stage_path_const = stage_path_constants::DREAMLAND,
            31 => stage_path_const = stage_path_constants::BATTLEFIELD,
            32 => stage_path_const = stage_path_constants::FINAL_DESTINATION,
            _ => stage_path_const = stage_path_constants::FOUNTAIN_OF_DREAMS,
        }

        let mut stage_polyline_arg:str = "";
        for point in stage_path_const.polyline{
            stage_polyline_arg += point[0] + ',' + point[1] + ' ';
        }
        stage_paths.unwrap().append(Polyline::new().set("points", stage_polyline_arg));
        for plat in stage_path_const.platforms{
            d = Data::new()
                .move_to((
                    plat[0][0] + (max_x / 2.0),
                    max_y - (plat[0][1] + (max_y / 2.0))))
                .line_to((
                    plat[1][0] + (max_x / 2.0),
                    max_y - (plat[1][1] + (max_y / 2.0)))
                );
            stage_paths.unwrap().append(Path::new().set("d", d)); 
        }
        dbg!(stage_polyline_arg);
        doc.unwrap().append(stage_paths);
        svg::save(format!("test-map-{}.svg", stage), &doc).unwrap();
    }
    Ok(())
}
