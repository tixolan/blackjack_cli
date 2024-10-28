use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};
use homedir::my_home;

pub struct Profile {
    pub credits: i32,
    pub games: i32,
    pub log: Vec<i32>,
}
pub fn load_profile() -> Result<Profile, Error>{
    let home_dir = my_home().expect("Couldn't Access Home Directory").unwrap();
    let file_path = home_dir.join(".blackjack_cli/profile.txt");
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);
    let mut profile: Profile = Profile {
        credits: 0,
        games: 0,
        log: vec![],
    };
    let lines: Vec<String> = buffered.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let credit_parts: Vec<&str> = lines[0].split(":").collect();
    profile.credits = credit_parts[1].parse::<i32>().unwrap();
    let game_parts: Vec<&str> = lines[1].split(":").collect();
    profile.games = game_parts[1].parse::<i32>().unwrap();

    profile.log = lines[3..].iter().map(| l | -> i32 { l.parse::<i32>().unwrap() }).collect();
    Ok(profile)
}

pub fn store_profile(profile: Profile) -> Result<(), Error> {
    let home_dir = my_home().expect("Couldn't Access Home Directory").unwrap();
    let file_path = home_dir.join(".blackjack_cli/profile.txt");
    let mut file = File::create(&file_path)?;
    write!(file, "credits:{}\ngames:{}\nlog:\n", profile.credits, profile.games)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;
    for entry in profile.log {
        if entry < 0 {
            writeln!(file, "-{}", entry)?;
        } else {
            writeln!(file, "+{}", entry)?;
        }
    }
    Ok(())
}

