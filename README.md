# Root Predictor
Thing to predict the outcome of Root games. Not very accurate.
## Usage
To run the project, go to the directory where the `games.root` file is located, then run the command `cargo run <subcommand>`.
## Subcommands
### get
Syntax: `cargo run get <game_id>`
Get a summary of a game.
### add [currently broken]
Syntax: `cargo run add`<br>
don't worry about this one for now
### predict
Syntax: `cargo run predict`<br>
First, enter the map that the game was played on.<br>
Then, enter the factions that each seat is playing, seperated by spaces.<br>
Then, enter the players playing each seat.<br>
Example:
```
> cargo run predict
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/root predict`
Enter map: AUTUMN
Enter factions: CATS EYRIE ALLIANCE VAGABOND
Enter player names: ANON ANON ANON ANON
CATS, played by ANON, has a ~24% chance of victory.
EYRIE, played by ANON, has a ~25% chance of victory.
ALLIANCE, played by ANON, has a ~25% chance of victory.
VAGABOND, played by ANON, has a ~24% chance of victory.
The predicted winner is ALLIANCE played by ANON.
```
Sometimes, the predicion might even be accurate!
