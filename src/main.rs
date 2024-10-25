use std::io::Write;
use std::thread::sleep;
use std::time;
use crate::cards::{Symbols::ACE, Card, display_cards, gen_deck};

mod cards;

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn calculate_score(hand: &Vec<Card>) -> i32 {
    let mut score = 0;
    let mut aces = 0;
    for card in hand {
        score += card._symbol.score();
        if card._symbol == ACE {
            aces += 1;
        }
    }
    while aces > 0 && score > 21 {
        score -= 10;
        aces -= 1;
    }

    score
}

fn print_game(player_stood: bool, dealer_hand: &mut Vec<Card>, player_hand: &Vec<Card>) {
    if player_stood {
        println!("Dealer's Hand (score = {}):", calculate_score(dealer_hand));
        display_cards(dealer_hand, false);
    } else {
        let last_card = dealer_hand.pop().unwrap();
        println!("Dealer's Hand (score = {} + ?):", calculate_score(dealer_hand));
        display_cards(dealer_hand, true);
        dealer_hand.push(last_card);
    }
    println!();
    println!("Player's Hand (score = {}):", calculate_score(player_hand));
    display_cards(player_hand, false);
}

fn print_log(log: &Vec<String>) {
    for entry in log {
        println!("{}", entry);
    }
}

fn main() -> std::io::Result<()> {
    let mut deck = gen_deck();
    let stdin = std::io::stdin();

    let mut log: Vec<String> = vec![];
    let mut player_hand: Vec<Card> = vec![];
    let mut dealer_hand: Vec<Card> = vec![];

    let mut player_stood = false;
    let mut game_over = false;

    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());

    // Check early Blackjack
    let score = calculate_score(&player_hand);
    if score == 21 {
        print_game(true, &mut dealer_hand, &player_hand);
        println!("Natural Blackjack! You won!");
        return Ok(());
    }

    // Game Loop
    loop {
        let mut line = String::new();
        clear_screen();
        print_game(player_stood, &mut dealer_hand, &player_hand);
        print_log(&log);

        if game_over { break }

        let player_score = calculate_score(&player_hand);
        let dealer_score = calculate_score(&dealer_hand);
        if player_score > 21 {
            print_game(true, &mut dealer_hand, &player_hand);
            log.push(String::from("Bust! You Lose!"));
            game_over = true;
            continue
        }
        if dealer_score > 21 {
            print_game(true, &mut dealer_hand, &player_hand);
            log.push(String::from("Dealer Busts! You Win!"));
            game_over = true;
            continue
        }

        if !player_stood {
            print!(">");
            // Print ">" immediately
            std::io::stdout().flush().expect("Couldn't Flush");

            stdin.read_line(&mut line).expect("Expected a command");
            if line.trim() == "hit" {
                player_hand.push(deck.pop().unwrap());
                log.push(String::from("Player Hits"))
            } else if line.trim() == "stand" {
                player_stood = true;
                log.push(String::from("Player Stands"))
            }
        } else {
            sleep(time::Duration::from_millis(1000));
            if dealer_score < 17 {
                dealer_hand.push(deck.pop().unwrap());
                log.push(String::from("Dealer Hits"));
            } else {
                log.push(String::from("Dealer Stands"));
                if player_score > dealer_score {
                    log.push(String::from("You Win!"));
                } else if player_score < dealer_score {
                    log.push(String::from("You Lose!"));
                } else {
                    log.push(String::from("Tie!"));
                }
                game_over = true;
            }
        }
    }

    Ok(())
}
