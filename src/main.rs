use std::fs::File;
use std::env;
use std::io::prelude::*;

fn mean(numbers: Vec<f64>) -> f64 { // calculate average
    let mut sum = 0.0;
    for x in &numbers {
        sum = sum + x;
    }
    return sum / numbers.len() as f64;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args[1] == "get" {
        let mut file = File::open("/etc/root/games.root")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let cmds: Vec<&str> = contents.split('\n').collect();
        let mut start = false;
        let mut players: Vec<&str> = vec![];
        let mut factions: Vec<&str> = vec![];
        let mut points: Vec<&str> = vec![];
        let mut notes: Vec<String> = vec![];
        let mut victor = "";
        let mut game = "";
        let mut map = "";
        let mut date = "";
        for cmd in cmds {
            let token: Vec<&str> = cmd.split(' ').collect();
            if start {
                if token[0] == "END" {
                    break;
                }
                if token[0] == "FACTIONS" {
                    let mut cont = true;
                    for t in token {
                        if cont {
                            cont = false;
                            continue;
                        }
                        factions.push(t);
                    }
                    continue;
                }
                if token[0] == "PLAYERS" {
                    let mut cont = true;
                    for t in token {
                        if cont {
                            cont = false;
                            continue;
                        }
                        players.push(t);
                    }
                    continue;
                }
                if token[0] == "POINTS" {
                    let mut cont = true;
                    for t in token {
                        if cont {
                            cont = false;
                            continue;
                        }
                        points.push(t);
                    }
                    continue;
                }
                if token[0] == "VICTOR" {
                    victor = token[1];
                    continue;
                }
                if token[0] == "NOTES" {
                    let mut cont = true;
                    let mut tempnote = String::new();
                    for t in token {
                        if cont {
                            cont = false;
                            continue;
                        }
                        if t == "|" {
                            notes.push(tempnote);
                            tempnote = String::new();
                        } else {
                            tempnote = format!("{} {}", tempnote, t);
                        }
                    }
                }
             } else if token[0] == "START" {
                if token[1] == args[2] || token[2] == args[2] {
                    start = true;
                    game = token[1];
                    date = token[2];
                    map = token[3];
                }
            }
        }
        println!("Game {} was played on the {} map at date {}.", game, map, date);
        for i in 0..factions.len() {
            println!("Player in seat {} was {} and played {}, scoring {} points.", i + 1, players[i], factions[i], points[i]);
        }
        println!("The {} won.", victor);
        println!("Interesting notes:");
        for note in notes {
            println!("-{}", note);
        }
    }
    if args[1] == "add" {
        let mut file = File::create("/etc/root/games.root")?;
        let mut data: String;
        let mut input = String::new();
        print!("Enter game ID: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("START {}", input.to_uppercase());
        input = String::new();
        print!("Enter game date: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{} {}", data, input.to_uppercase());
        input = String::new();
        print!("Enter map: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{} {}", data, input.to_uppercase());
        input = String::new();
        print!("Enter factions: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{}\nFACTIONS {}", data, input.to_uppercase());
        input = String::new();
        print!("Enter player names: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{}\nPLAYERS {}", data, input.to_uppercase());
        input = String::new();
        print!("Enter points: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{}\nPOINTS {}", data, input.to_uppercase());
        input = String::new();
        print!("Enter winner: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        data = format!("{}\nVICTOR {}\nNOTES", data, input.to_uppercase());
        input = String::new();
        loop {
            print!("Enter an interesting note about the game, or a \"/\" to stop entering notes: ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut input)?;
            input = input.trim().to_string();
            std::io::stdout().flush()?;
            if input == "/" {
                data = format!("{}\nEND", data);
                break;
            }
            data = format!("{} {} |", data, input);
            input = String::new();
        }
        file.write_all(data.as_bytes())?;
    }
    if args[1] == "predict" {
        let mut mapinput = String::new();
        let mut playerinput = String::new();
        let mut factioninput = String::new();
        let map: String;
        let factions: Vec<&str>;
        let players: Vec<&str>;
        print!("Enter map: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut mapinput)?;
        mapinput = mapinput.trim().to_string();
        map = mapinput.to_uppercase();
        print!("Enter factions: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut playerinput)?;
        playerinput = playerinput.trim().to_string().to_uppercase();
        factions = playerinput.split(' ').collect();
        print!("Enter player names: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut factioninput)?;
        factioninput = factioninput.trim().to_string().to_uppercase();
        players = factioninput.split(' ').collect();
        let mut file = File::open("/etc/root/games.root")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let cmds: Vec<&str> = contents.split('\n').collect();
        let mut players_data: Vec<&str> = vec![];
        let mut factions_data: Vec<&str> = vec![];
        let mut seats: Vec<i32> = vec![];
        let mut points: Vec<i32> = vec![];
        let mut maps: Vec<&str> = vec![];
        let mut map_data = "";
        for cmd in cmds {
            let token: Vec<&str> = cmd.split(' ').collect();
            if token[0] == "START" {
                map_data = token[3];
            }
            if token[0] == "FACTIONS" {
                let mut cont = 0;
                for t in token {
                    if cont == 0 {
                        cont += 1;
                        continue;
                    }
                    factions_data.push(t);
                    seats.push(cont);
                    maps.push(&map_data);
                    cont += 1;
                }
                continue;
            }
            if token[0] == "PLAYERS" {
                let mut cont = true;
                for t in token {
                    if cont {
                        cont = false;
                        continue;
                    }
                    players_data.push(t);
                }
                continue;
            }
            if token[0] == "POINTS" {
                let mut cont = true;
                for t in token {
                    if cont {
                        cont = false;
                        continue;
                    }
                    points.push(t.parse().expect("idk"));
                }
                continue;
            }
        }
        let mut moddedpts: Vec<f64> = vec![];
        for i in 0..factions.len() {
            let mut biasedpts: Vec<f64> = vec![];
            let player = players[i];
            let faction = factions[i];
            let seat = i as i32;
            for ii in 0..points.len() {
                let mut mult = 0;
                if player == players_data[ii] {
                    mult += 2;
                }
                if faction == factions_data[ii] {
                    mult += 3;
                }
                if seat + 1 == seats[ii] {
                    mult += 1;
                }
                if map == maps[ii] {
                    mult += 1;
                }
                let pts = points[ii] as f64;
                for _ in 0..mult {
                    biasedpts.push(pts);
                }
            }
            moddedpts.push(mean(biasedpts));
        }
        let mut winner_fact = "NO-ONE";
        let mut winner_play = "NO-ONE";
        let mut winchance = 0.0;
        let mut finalpts: Vec<f64> = vec![];
        for p in &moddedpts {
            finalpts.push(*p);
        }
        let mut all = 0.0;
        for p in &finalpts {
            all += p;
        }
        for i in 0..moddedpts.len() {
            let chance = finalpts[i] / all * 100.0;
            println!("{}, played by {}, has a ~{}% chance of victory.", factions[i], players[i], chance as i32);
            if chance > winchance {
                winner_fact = factions[i];
                winner_play = players[i];
                winchance = chance;
            }
        }
        println!("The predicted winner is {} played by {}.", winner_fact, winner_play);
    }
    Ok(())
}
