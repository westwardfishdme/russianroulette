use rand::{thread_rng, Rng};
use std::{io, thread::sleep, time};
/* Russian Roulette */
// ai player
fn ai_dialogue(choice: u8) {
    let dialogue: &str;

    match choice {
        0 => dialogue = "I choose you.",
        1 => dialogue = "I choose to shoot myself.",
        2 => dialogue = "It appears that there is only one chamber left... I'm sorry, but it would deem that fate was not on your side.",
        _ => todo!(),
    }
    println!("{}", dialogue);
    sleep(time::Duration::from_millis(1000));
}
// cool little wait timer.
fn timer_wait(char_or_str: &str, len: u8) {
    println!("\n*click*");
    for _i in 0..len {
        print!("{}", char_or_str);
        io::Write::flush(&mut io::stdout()).expect("failed to flush output");
        let timer_ms = time::Duration::from_millis(600);
        sleep(timer_ms);
    }
}

// ai code.
fn fatehimself(tries_remaining: u8, bullet: u8) -> u8 {
    let mut random = thread_rng();
    let aim = random.gen_range(0..2);
    if tries_remaining == 1 {
        ai_dialogue(2);
        return 1;
    } else {
        ai_dialogue(aim);
        timer_wait(".", 3);
    }

    if tries_remaining == bullet && aim == 0 {
        return 1;
    } else if tries_remaining == bullet && aim == 1 {
        return 0;
    } else if tries_remaining != bullet {
        println!("\nThe trigger is pulled, but nothing happens.\nYour opponent passes the gun back to you.\nThere are {} remaining chamber(s) left to go through.", tries_remaining-1);
        return 2;
    } else {
        return 2;
    }
}

// player code
pub fn russianroulette() -> u8 {
    let mut random = thread_rng();
    let bullet_pos: u8 = random.gen_range(1..7);
    let mut tries: u8 = 6;
    let mut aim_input = String::new();
    let mut wincon = 2;

    // detect player input to determine the player's move
    while tries != 0 {
        aim_input.clear();
        let mut aim: u8 = 3;
        while aim == 3 {
            println!("\nChoose a target: Opponent or self: ");
            io::stdin()
                .read_line(&mut aim_input)
                .expect("that isn't right...");

            match aim_input.as_str().to_lowercase().trim() {
                "self" => aim = 0, //player = 0, ai = 1
                "opponent" | "opp" => aim = 1,
                _ => println!("that isn't right..."),
            }
            aim_input.clear();
        }
        timer_wait(".", 3);
        // on players turn
        if aim == 0 && tries == bullet_pos {
            sleep(time::Duration::from_millis(600));
            wincon = 1;
            break;
        } else if tries == bullet_pos && aim == 1 {
            wincon = 0;
            break;
        } else {
            println!(
                "\nThe trigger is pulled, but nothing happens, and you pass the gun to the player sitting across from you.\nThere are {} chamber(s) left to go through.\n",
                tries - 1
            );
            tries -= 1;
        }

        // on ai turn
        match fatehimself(tries, bullet_pos) {
            0 => wincon = 0,
            1 => wincon = 1,
            2 => tries -= 1,
            _ => todo!(),
        }
        if wincon == 1 || wincon == 0 {
            break;
        }
    }
    match wincon{
        0=>println!("\nYour opponent lies dead upon the table. \nYou solemnly leave the room, thinking about how his fate could have easily been yours."),
        1=>println!("\nBefore you know it, you lie dead upon the table. \nIt would seem that your fate was sealed the moment you squeezed the trigger."),
        _=>todo!(),

    }
    sleep(time::Duration::from_millis(2000));
    return wincon;
}
