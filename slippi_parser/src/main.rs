use csv::Writer;
use std::{fs, io, error::Error,fs::File};
use std::io::Seek;
use std::io::Write;
use peppi::io::slippi::read;
use peppi::frame::Rollbacks;


// `ssbm-data` provides enums for characters, stages, action states, etc.
use ssbm_data::action_state::Common::{*};


//stole a lot from https://github.com/project-slippi/slippi-js/blob/master/src/stats/combos.ts#L7


fn main() -> Result<(), Box<dyn Error>>{
    let mut zip = zip::ZipArchive::new(File::open("tests/testfolder.zip").unwrap()).unwrap();


    // Create a new CSV file
    let mut wtr = Writer::from_path("combos.csv")?;
    wtr.write_record(&["Start x","Start y","End x","End y","Start Move","End Move","Comboer Character","Comboee Character","Start %","End %","Frames Between Moves","Stage"])?;


    // Iterate over each file in the zip archive
    let total_iterations = zip.len();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // Check if the file is a .slp file
        if outpath.extension().unwrap_or_default() == "slp" {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            // Extract the file
            let mut outfile = File::create_new(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
            //println!("Extracted {}", outpath.display());
            print!("\r[");
            let progress = (i as f64 / total_iterations as f64 * 50.0) as usize; // Adjust 50 for the desired length of the progress bar
            for _ in 0..progress {
                print!("=");
            }
            for _ in progress..50 {
                print!(" ");
            }
            print!("] {}%", (i as f64 / total_iterations as f64 * 100.0) as u32);
            // Flush the output to ensure the progress is visible
            std::io::stdout().flush().unwrap();
            outfile.rewind().unwrap();


            let mut r = io::BufReader::new(outfile);
            let game = read(&mut r, None).unwrap();
            let stage = game.start.stage;
            let rollbacks = game.frames.rollbacks(Rollbacks::ExceptLast);
            let mut start_frame: [usize;2] = [0,0];
            for frame_idx in 1..game.frames.len() {
                if rollbacks[frame_idx]{
                    continue;
                }
                for (port_idx, port_data) in game.frames.ports.iter().enumerate() {
                    let state=port_data.leader.post.state.get(frame_idx).unwrap_or(0);


                    let last_hit_by_instance = <Option<arrow2::array::PrimitiveArray<u16>> as Clone>::clone(&port_data.leader.post.last_hit_by_instance).unwrap_or_default().get(frame_idx-1).unwrap_or(0);
                    let hit_by_instance = <Option<arrow2::array::PrimitiveArray<u16>> as Clone>::clone(&port_data.leader.post.last_hit_by_instance).unwrap_or_default().get(frame_idx).unwrap_or(0);


                    let last_state=port_data.leader.post.state.get(frame_idx-1).unwrap_or(0);
                    let in_hitstun= is_damaged(last_state)||is_grabbed(last_state)||is_command_grabbed(last_state);
                    let character = port_data.leader.post.character.get(frame_idx).unwrap_or(0);
                    let opp_character = game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.character.get(frame_idx).unwrap_or(0);


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
                    if hit_by_instance > last_hit_by_instance {
                        let opponent_attack= game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(frame_idx).unwrap_or(0);
                        if !is_pummel_or_throw(opponent_attack) {
                            if in_hitstun{
                                /*
                                println!("{} comboed on frame {} by instance {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap(), hit_by_instance);
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
                                //println!("{} raw hit on frame {} by instance {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap(), hit_by_instance);
                            }
                        }
                        start_frame[port_idx]=frame_idx;
                    }
                }
            }


            fs::remove_file(&outpath).unwrap();
        }
    }


    // Flush and close the writer
    wtr.flush()?;


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
/*
fn is_attack(state: u16) -> bool{
    return
        (state >= Attack11 as u16 && state <= LandingAirLw as u16) ||
        state>=341  //>=341 is character specific things such as specials
                    //There's definitely a better way of doing this...
    ;
}
*/
fn is_pummel_or_throw(state: u8) -> bool{
    return
        (state >= 52 && state <= 60) || state == 0
    ;
}

