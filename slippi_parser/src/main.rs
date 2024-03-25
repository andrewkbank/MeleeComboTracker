use std::{fs, io};
use peppi::io::slippi::read;
use peppi::frame::Rollbacks;

// `ssbm-data` provides enums for characters, stages, action states, etc.
// You can just hard-code constants instead, if you prefer.
use ssbm_data::action_state::Common::{self, *};


//stole a lot from https://github.com/project-slippi/slippi-js/blob/master/src/stats/combos.ts#L7

fn main() {
    let mut r = io::BufReader::new(fs::File::open("tests/test1.slp").unwrap());
    let game = read(&mut r, None).unwrap();

    let mut is_comboed = vec![vec ![false; game.frames.len()]; game.frames.ports.len()];
    let rollbacks = game.frames.rollbacks(Rollbacks::ExceptLast);
    for frame_idx in 1..game.frames.len() {
        if rollbacks[frame_idx]{
            continue;
        }
        for (port_idx, port_data) in game.frames.ports.iter().enumerate() {
            let damage_taken = port_data.leader.post.percent.get(frame_idx).unwrap_or(0.0) - port_data.leader.post.percent.get(frame_idx-1).unwrap_or(0.0);
            let state=port_data.leader.post.state.get(frame_idx).unwrap_or(0);

            //track hits
            if damage_taken > 0.0 {
                println!(
                    "{} hit on frame {}", 
                    game.start.players[port_idx].port,
                    game.frames.id.get(frame_idx).unwrap()
                );
            }
            
            //track hitstun
            //state list here: https://docs.rs/ssbm-data/latest/ssbm_data/action_state/enum.Common.html
            if is_damaged(state)||is_grabbed(state)||is_command_grabbed(state){
                is_comboed[port_idx][frame_idx] = true;
                println!(
                    "{} on frame {} in animation {}",
                    game.start.players[port_idx].port,
                    game.frames.id.get(frame_idx).unwrap(),
                    state
                );
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

fn is_attack(state: u16) -> bool{
    return
        (state >= Attack11 as u16 && state <= LandingAirLw as u16) ||
        state>=341  //>=341 is character specific things such as specials
                    //There's definitely a better way of doing this...
    ;
}
fn is_grab(state: u16) -> bool{
    return 
        state >= Catch as u16 && state <= ThrowLw as u16
    ;
}