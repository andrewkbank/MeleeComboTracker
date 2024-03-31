use std::{fs, io};
use peppi::io::slippi::read;
use peppi::frame::Rollbacks;
//use arrow2::array::PrimitiveArray;

// `ssbm-data` provides enums for characters, stages, action states, etc.
// You can just hard-code constants instead, if you prefer.
use ssbm_data::action_state::Common::{*};


//stole a lot from https://github.com/project-slippi/slippi-js/blob/master/src/stats/combos.ts#L7

fn main() {
    let mut r = io::BufReader::new(fs::File::open("tests/test3.slp").unwrap());
    let game = read(&mut r, None).unwrap();
    let metadata = game.metadata;
    match metadata {
        Some(map) => {
            // If Some, print the contents of the HashMap
            for (key, value) in map.iter() {
                println!("Key: {}, Value: {:?}", key, value);
            }
        }
        None => {
            println!("Option is None");
        }
    }
    let rollbacks = game.frames.rollbacks(Rollbacks::ExceptLast);
    let mut last_attack: [u8;2] = [0,0];
    for frame_idx in 1..game.frames.len() {
        if rollbacks[frame_idx]{
            continue;
        }
        for (port_idx, port_data) in game.frames.ports.iter().enumerate() {
            //let damage_taken = port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0) - port_data.leader.post.percent.get(frame_idx-1).unwrap_or(0.0);

            let state=port_data.leader.post.state.get(frame_idx).unwrap_or(0);

            let last_hit_by_instance = <Option<arrow2::array::PrimitiveArray<u16>> as Clone>::clone(&port_data.leader.post.last_hit_by_instance).unwrap_or_default().get(frame_idx-1).unwrap_or(0);
            let hit_by_instance = <Option<arrow2::array::PrimitiveArray<u16>> as Clone>::clone(&port_data.leader.post.last_hit_by_instance).unwrap_or_default().get(frame_idx).unwrap_or(0);
            
            //let position = port_data.leader.post.position;

            let last_state=port_data.leader.post.state.get(frame_idx-1).unwrap_or(0);
            let in_hitstun= is_damaged(last_state)||is_grabbed(last_state)||is_command_grabbed(last_state);

            //track getting grabbed
            if is_grabbed(state) && !is_grabbed(port_data.leader.post.state.get(frame_idx-1).unwrap_or(0)){   //grabbed this frame and not the last frame
                if in_hitstun {
                    println!(
                        "{} combo grabbed on frame {}", 
                        game.start.players[port_idx].port,
                        game.frames.id.get(frame_idx).unwrap()
                    );
                    println!(
                        "{} comboed into grab",
                        last_attack[port_idx]
                    );
                }else{
                    println!(
                        "{} raw grabbed on frame {}", 
                        game.start.players[port_idx].port,
                        game.frames.id.get(frame_idx).unwrap()
                    );
                }
                
            }else 
            //track hits
            if hit_by_instance != last_hit_by_instance {
                let mut opponent = port_data.leader.post.last_hit_by.get(frame_idx).unwrap_or(0);

                //special case for opponent=6
                //https://github.com/project-slippi/slippi-js/pull/71
                //assuming 2 players
                if opponent==6 {
                    //println!("OPPONENT 6 on frame {}",game.frames.id.get(frame_idx).unwrap());
                    opponent = (port_idx as u8+1)%2;
                    //println!("Opponent chosen: {}",opponent);
                }else{
                    if opponent as usize>port_idx{
                        opponent=1;
                    }else{
                        opponent=0;
                    }
                }
                //let opponent_move = game.frames.ports[opponent as usize].leader.post.state.get(frame_idx).unwrap_or(0);
                let opponent_attack= game.frames.ports[opponent as usize].leader.post.last_attack_landed.get(frame_idx).unwrap_or(0);
                if !is_pummel_or_throw(opponent_attack) {
                    if in_hitstun{
                        println!(
                            "{} comboed on frame {} by instance {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap(), hit_by_instance
                        );
                        println!(
                            "{} comboed into {}",
                            last_attack[port_idx],
                            opponent_attack
                        );
                    }else{
                        println!(
                            "{} raw hit on frame {} by instance {}", 
                            game.start.players[port_idx].port,
                            game.frames.id.get(frame_idx).unwrap(),
                            hit_by_instance
                        );
                    }
                }
                last_attack[port_idx]=opponent_attack;
            }
        }
    }
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
        state >= 52 && state <= 60
    ;
}