use csv::Writer;
use std::{fs, io, error::Error,fs::File};
use std::io::Seek;
use std::io::Write;
use sevenz_rust::{Archive, BlockDecoder, Password};
use peppi::io::slippi::read;
use peppi::frame::Rollbacks;

use std::fmt;

// Define a custom error type
#[derive(Debug)]
struct CustomError(String);

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// `ssbm-data` provides enums for characters, stages, action states, etc.
use ssbm_data::action_state::Common::{*};


//stole a lot from https://github.com/project-slippi/slippi-js/blob/master/src/stats/combos.ts#L7


fn main() -> Result<(), Box<dyn Error>>{
    //let mut zip = zip::ZipArchive::new(File::open("tests/Slippi Dumps.zip").unwrap()).unwrap();
    //let mut massive_fucking_file = File::open("tests/ranked-anonymized.7z").unwrap();

    // Create a new CSV file
    let mut wtr = Writer::from_path("combos.csv")?;
    wtr.write_record(&["Start x","Start y","End x","End y","Start Move","End Move","Comboer Character","Comboee Character","Start %","End %","Frames Between Moves","Stage"])?;

    //parse_zip_file(zip,&mut wtr);
    parse_slp_file(fs::File::open("tests/test7.slp").unwrap(),&mut wtr);
    //parse_7z_file(massive_fucking_file,&mut wtr);
    // Flush and close the writer
    wtr.flush()?;


    Ok(())
}

fn parse_slp_file(mut file: fs::File, wtr: &mut Writer<fs::File>) -> Result<(), Box<dyn Error>>{
    //filter out super small and super large slp files
    /*
    if file.metadata().unwrap().len()<5000 ||file.metadata().unwrap().len()>15000000{
        return Ok(());
    }
    */
    let mut r = io::BufReader::new(file);
    let game = read(&mut r, None)?;
    if game.start.players.len()!=2{
        return Ok(());
    }
    let stage = game.start.stage;
    let rollbacks = game.frames.rollbacks(Rollbacks::ExceptLast);
    let mut start_frame: [usize;2] = [0,0];
    let mut prevent_multihits: [bool;2] = [true,true];
    for frame_idx in 1..game.frames.len() {
        if rollbacks[frame_idx]{
            continue;
        }
        for (port_idx, port_data) in game.frames.ports.iter().enumerate() {
            let state=port_data.leader.post.state.get(frame_idx).unwrap_or(0);

            let last_hitstun_remaining = <Option<arrow2::array::PrimitiveArray<f32>> as Clone>::clone(&port_data.leader.post.misc_as).unwrap_or_default().get(frame_idx-1).unwrap_or(0.0);
            let hitstun_remaining = <Option<arrow2::array::PrimitiveArray<f32>> as Clone>::clone(&port_data.leader.post.misc_as).unwrap_or_default().get(frame_idx).unwrap_or(0.0);

            let last_state=port_data.leader.post.state.get(frame_idx-1).unwrap_or(0);
            let in_hitstun= is_damaged(last_state)||is_grabbed(last_state)||is_command_grabbed(last_state);
            let character = port_data.leader.post.character.get(frame_idx).unwrap_or(0);
            let opp_character = game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.character.get(frame_idx).unwrap_or(0);


            //figure out if we should reset the multihit flag
            //reset it if we exit hitstun or if the opponent's state changes (ie: they stop their multihit)
            let opp_state_age = <Option<arrow2::array::PrimitiveArray<f32>> as Clone>::clone(&game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state_age).unwrap_or_default().get(frame_idx).unwrap_or(0.0);
            let last_opp_state_age = <Option<arrow2::array::PrimitiveArray<f32>> as Clone>::clone(&game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state_age).unwrap_or_default().get(frame_idx-1).unwrap_or(0.0);
            if !prevent_multihits[port_idx] && (!in_hitstun || (opp_state_age < last_opp_state_age && opp_state_age<5.0)){
                prevent_multihits[port_idx]=true;
                /*
                if opp_state_age < last_opp_state_age{
                    println!("state 1: {}, State 2: {}, state_age: {}, frame {}",
                    game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state.get(frame_idx-2).unwrap_or(0),
                    game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state.get(frame_idx).unwrap_or(0),
                    opp_state_age,
                    game.frames.id.get(frame_idx).unwrap()
                );
                }else{
                    println!("hitstun {} ended frame {}",in_hitstun, game.frames.id.get(frame_idx).unwrap());
                }
                */
            }
            let opponent_attack= game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(frame_idx).unwrap_or(0);
            /*
            if is_pummel_or_throw(opponent_attack) && opponent_attack != 0{
                println!("throw: {} on frame {}",opponent_attack,game.frames.id.get(frame_idx).unwrap());
                if hitstun_remaining > last_hitstun_remaining{
                    println!("hitstun increase on frame {}",game.frames.id.get(frame_idx).unwrap());
                }
                if port_data.leader.post.percent.get(frame_idx-1).unwrap_or(0.0)<port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0){
                    println!("percent increase on frame {}",game.frames.id.get(frame_idx).unwrap());
                }

            }
            */
            //track getting grabbed
            if is_grabbed(state) && !is_grabbed(port_data.leader.post.state.get(frame_idx-1).unwrap_or(0)){   //grabbed this frame and not the last frame
                if in_hitstun {
                    /*
                    println!("{} combo grabbed on frame {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap());
                    println!("Start position: ({},{}), End position: ({},{})", port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0),port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0));
                    println!("{} comboed into grab", game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0));
                    println!("character {} comboed by {}",character,opp_character);
                    println!("Start %: {}, End %: {}",port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0),port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0));
                    println!("Frames between moves: {}",(frame_idx-start_frame[port_idx]));
                    println!("Stage: {}",stage);
                    println!("---------------");
                    */
                    let write_data = [
                        port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0).to_string(),                                                 //start x
                        port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0).to_string(),                                                 //start y
                        port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0).to_string(),                                                             //end x
                        port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0).to_string(),                                                             //end y
                        game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0).to_string(),  //start move
                        0.to_string(),                                                                                                                          //end move (grab)
                        opp_character.to_string(),                                                                                                              //comboer character
                        character.to_string(),                                                                                                                  //comboee character
                        port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0).to_string(),                                                  //start % (before the start move hits)
                        port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0).to_string(),                                                                //end %
                        (frame_idx-start_frame[port_idx]).to_string(),                                                                                          //frames between start and end move
                        stage.to_string()                                                                                                                       //stage
                    ];
                    wtr.write_record(&write_data)?;
                }else{
                    //println!("{} raw grabbed on frame {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap());
                }
               
            }else
            //track hits

            //instead of using hit_by_instance, we can check hits if hitstun increases + percent increases
            if ((hitstun_remaining > last_hitstun_remaining && port_data.leader.post.percent.get(frame_idx-1).unwrap_or(0.0)<port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0))
                || (is_throw(opponent_attack) && opp_state_age<30.0 && (hitstun_remaining>last_hitstun_remaining|| port_data.leader.post.percent.get(frame_idx-1).unwrap_or(0.0)<port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0))))
                && prevent_multihits[port_idx]{
                //let opponent_attack= game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(frame_idx).unwrap_or(0);
                if !is_pummel_or_throw(opponent_attack) {
                    if in_hitstun{
                        /*
                        println!("{} comboed on frame {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap());
                        //println!("start state: {}, end state: {}",game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state.get(start_frame[port_idx]-1).unwrap_or(0),game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.state.get(frame_idx).unwrap_or(0));
                        println!("Start position: ({},{}), End position: ({},{})", port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0),port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0));
                        println!("{} comboed into {}",game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0),opponent_attack);
                        println!("character {} comboed by {}",character,opp_character);
                        println!("Start %: {}, End %: {}",port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0) as u16,port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0) as u16);
                        println!("Frames between moves: {}",(frame_idx-start_frame[port_idx]));
                        println!("Stage: {}",stage);
                        println!("---------------");
                        */
                        let write_data = [
                            port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0).to_string(),                                                 //start x
                            port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0).to_string(),                                                 //start y
                            port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0).to_string(),                                                             //end x
                            port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0).to_string(),                                                             //end y
                            game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0).to_string(),  //start move
                            opponent_attack.to_string(),                                                                                                            //end move (grab)
                            opp_character.to_string(),                                                                                                              //comboer character
                            character.to_string(),                                                                                                                  //comboee character
                            port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0).to_string(),                                                  //start % (before the start move hits)
                            port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0).to_string(),                                                                //end %
                            (frame_idx-start_frame[port_idx]).to_string(),                                                                                          //frames between start and end move
                            stage.to_string()                                                                                                                       //stage
                        ];
                        wtr.write_record(&write_data)?;
                    }else{
                        //println!("{} raw hit on frame {}: attack: {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap(),opponent_attack);
                    }
                }
                prevent_multihits[port_idx]=false;
                start_frame[port_idx]=frame_idx;
            }
        }
    }

    Ok(())
}

fn parse_zip_file(mut zip: zip::ZipArchive<std::fs::File>, wtr: &mut Writer<fs::File>) -> Result<(), Box<dyn Error>>{
    // Iterate over each file in the zip archive
    let total_iterations = zip.len();
    for i in 0..zip.len() {
        //progress bar
        print!("\r[");
        let progress = (i as f64 / total_iterations as f64 * 50.0) as usize; // Adjust 50 for the desired length of the progress bar
        for _ in 0..progress {
            print!("=");
        }
        for _ in progress..50 {
            print!(" ");
        }
        print!("] {}%", (i as f64 / total_iterations as f64 * 100.0) as u32);

        let mut file = zip.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // Check if the file is a .slp file
        if outpath.extension().unwrap_or_default() == "slp" {
            // Extract the file
            if std::path::Path::new("read.slp").exists() { 
                fs::remove_file("read.slp").unwrap();
            }
            let mut outfile = File::create_new("read.slp").unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
            
            // Flush the output to ensure the progress is visible
            std::io::stdout().flush().unwrap();
            outfile.rewind().unwrap();

            parse_slp_file(outfile,wtr);
        }else if outpath.extension().unwrap_or_default() == "7z" {
            println!("7z file: {}",outpath.file_name().unwrap().to_str().unwrap());
            if std::path::Path::new("inner_7z.7z").exists() { 
                fs::remove_file("inner_7z.7z").unwrap();
            }

            let mut inner_7z_file = File::create_new("inner_7z.7z").unwrap();
            std::io::copy(&mut file, &mut inner_7z_file).unwrap();

            // Flush the output to ensure the progress is visible
            std::io::stdout().flush().unwrap();
            inner_7z_file.rewind().unwrap();

            parse_7z_file(inner_7z_file,wtr);
        }else if outpath.extension().unwrap_or_default() == "zip" {
            println!("zip file: {}",outpath.file_name().unwrap().to_str().unwrap());

            if std::path::Path::new("inner_zip.zip").exists() { 
                fs::remove_file("inner_zip.zip").unwrap();
            }

            let mut inner_zip_file = File::create_new("inner_zip.zip").unwrap();
            std::io::copy(&mut file, &mut inner_zip_file).unwrap();

            // Flush the output to ensure the progress is visible
            std::io::stdout().flush().unwrap();
            inner_zip_file.rewind().unwrap();

            let mut next_zip = zip::ZipArchive::new(inner_zip_file).unwrap();
            
            parse_zip_file(next_zip,wtr);
        }
    }

    Ok(())
}

fn parse_7z_file(mut file: fs::File,wtr: &mut Writer<fs::File>) -> Result<(), Box<dyn Error>>{
    let len = file.metadata().unwrap().len();
    let password = Password::empty();
    let archive = Archive::read(&mut file, len, password.as_slice()).unwrap();
    let folder_count = archive.folders.len();
    for folder_index in 0..folder_count {
        let forder_dec = BlockDecoder::new(folder_index, &archive, password.as_slice(), &mut file);
        let mut i=0;
        let total_iterations = forder_dec.entry_count();
        print!("\x1B[1A\r[");
        let progress = (folder_index as f64 / folder_count as f64 * 50.0) as usize; // Adjust 50 for the desired length of the progress bar
        for _ in 0..progress {
            print!("=");
        }
        for _ in progress..50 {
            print!(" ");
        }
        println!("] {}%", (folder_index as f64 / folder_count as f64 * 100.0));
        std::io::stdout().flush().unwrap();
        forder_dec
            .for_each_entries(&mut |entry, reader| {
                print!("\r[");
                let progress = (i as f64 / total_iterations as f64 * 50.0) as usize; // Adjust 50 for the desired length of the progress bar
                for _ in 0..progress {
                    print!("=");
                }
                for _ in progress..50 {
                    print!(" ");
                }
                print!("] {}% File: {}", (i as f64 / total_iterations as f64 * 100.0) as u32,entry.name());
                std::io::stdout().flush().unwrap();
                i+=1;
                if std::path::Path::new(entry.name()).exists() { 
                    fs::remove_file(entry.name()).unwrap();
                }
                sevenz_rust::default_entry_extract_fn(entry, reader, &std::path::PathBuf::from(entry.name()))?;
                let slp_file = File::open(entry.name()).unwrap();
                match parse_slp_file(slp_file,wtr){
                    Ok(()) => {
                        fs::remove_file(entry.name()).unwrap();
                        print!("     processed");
                    },
                    Err(err) => {
                        fs::remove_file(entry.name()).unwrap();
                        print!(" not processed");
                        //return Ok(false);
                    },
                }
                Ok(true)
            })
            .expect("ok");
    }

    Ok(())
}

fn is_damaged(state: u16) -> bool{
    return
        (state >= DamageHi1 as u16 && state <= DamageFlyRoll as u16) ||   //17 different types of damage
        //state == DamageFall as u16 || //Tumble (I'm removing bc you can act out of tumble unlike the rest of these states)
        state == DownDamageU as u16 ||  //Jab Reset
        state == DownDamageD as u16 ||  //Also Jab Reset
        state == CaptureDamageHi as u16 || //pummel
        state == CaptureDamageLw as u16   //also pummel
    ;
}


fn is_grabbed(state: u16) -> bool{
    return state >= CapturePulledHi as u16 && state <= CaptureFoot as u16;
}


fn is_command_grabbed(state: u16) -> bool{
    return
        (state >= ShoulderedWait as u16 && state <= ThrownMewtwoAir as u16) ||
        (state >= CaptureKirbyYoshi as u16 && state <= CaptureLikeLike as u16)
    ;
}

fn is_pummel_or_throw(state: u8) -> bool{
    return
        (state >= 52 && state <= 60) || state == 0
    ;
}
fn is_throw(state:u8) -> bool{
    return state>=53 && state <= 60;
}
/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/