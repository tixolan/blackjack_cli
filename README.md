# Blackjack CLI
Something to play in your command line whenever you feel bored. I first
coded this project in Python but then got challenged to do it in Rust, so
I did.

## Compiling
To compile simply use cargo like so:
```
git clone https://github.com/tixolan/blackjack_cli
cd blackjack_cli
cargo build --release
```

## Configuration File Structure
In order to store the amount of credits the player has, the program must
store that information in a file. The format of this file is as follows:
```
credits: <amount-of-credits>
games: <amount-of-games>
log:
< +/- ><amount-of-credits>
```

## Future Plans
I plan on adding a betting system so you can bet a certain amount and save it.
