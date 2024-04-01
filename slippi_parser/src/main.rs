use std::{fs, io};
use peppi::io::slippi::read;
use peppi::frame::Rollbacks;

// `ssbm-data` provides enums for characters, stages, action states, etc.
// You can just hard-code constants instead, if you prefer.
use ssbm_data::action_state::Common::{*};

//stole a lot from https://github.com/project-slippi/slippi-js/blob/master/src/stats/combos.ts#L7

fn main() {
    let mut r = io::BufReader::new(fs::File::open("tests/test4.slp").unwrap());
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
                    println!("{} combo grabbed on frame {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap());
                    println!("Start position: ({},{}), End position: ({},{})", port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0),port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0));
                    println!("{} comboed into grab", game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0));
                    println!("character {} comboed by {}",character,opp_character);
                    println!("Start %: {}, End %: {}",port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0),port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0));
                    println!("Frames between moves: {}",(frame_idx-start_frame[port_idx]));
                    println!("Stage: {}",stage);
                    println!("---------------");
                }else{
                    println!("{} raw grabbed on frame {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap());
                }
                
            }else 
            //track hits
            if hit_by_instance != last_hit_by_instance {
                let opponent_attack= game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(frame_idx).unwrap_or(0);
                if !is_pummel_or_throw(opponent_attack) {
                    if in_hitstun{
                        println!("{} comboed on frame {} by instance {}", game.start.players[port_idx].port, game.frames.id.get(frame_idx).unwrap(), hit_by_instance);
                        println!("Start position: ({},{}), End position: ({},{})", port_data.leader.post.position.x.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.y.get(start_frame[port_idx]).unwrap_or(0.0),port_data.leader.post.position.x.get(frame_idx).unwrap_or(0.0),port_data.leader.post.position.y.get(frame_idx).unwrap_or(0.0));
                        println!("{} comboed into {}",game.frames.ports[((port_idx as u8+1)%2) as usize].leader.post.last_attack_landed.get(start_frame[port_idx]).unwrap_or(0),opponent_attack);
                        println!("character {} comboed by {}",character,opp_character);
                        println!("Start %: {}, End %: {}",port_data.leader.post.percent.get(start_frame[port_idx]-1).unwrap_or(0.0) as u16,port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0) as u16);
                        println!("Frames between moves: {}",(frame_idx-start_frame[port_idx]));
                        println!("Stage: {}",stage);
                        println!("---------------");
                    }else{
                        println!(
                            "{} raw hit on frame {} by instance {}", 
                            game.start.players[port_idx].port,
                            game.frames.id.get(frame_idx).unwrap(),
                            hit_by_instance
                        );
                    }
                }
                start_frame[port_idx]=frame_idx;
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