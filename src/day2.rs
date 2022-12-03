/*
Opponent input
A = Rock
B = Paper
C = Scissors

Player input
X = Rock
Y = Paper
Z = Scissors

part2 meaning 
X = lose
Y = tie
Z = win

Scoreing Rules:
1 for rock
2 for paper
3 for scissors
0 for loss 
3 for tie 
6 for win
*/
// declare constant str's
const ROCK_OPP: &str = "A";
const PAPER_OPP: &str = "B";
const SCISSORS_OPP: &str = "C";

const ROCK: &str = "X";
const PAPER: &str = "Y";
const SCISSORS: &str = "Z";

const LOSE: &str = "X";
const TIE: &str = "Y";
const WIN: &str = "Z";

const ROCK_SCORE: i128 = 1;
const PAPER_SCORE: i128 = 2;
const SCISSORS_SCORE: i128 = 3;

const LOSE_SCORE: i128 = 0;
const TIE_SCORE: i128 = 3;
const WIN_SCORE: i128 = 6;

pub fn run(input: String) -> (i128, i128) {
    let mut score: i128 = 0;
    let mut scorept2: i128 = 0;
    // part 1
    for line in input.lines() {
        let opponent = line[0..1].to_string();
        let player = line[2..3].to_string();
        let opponent = opponent.as_str();
        let player = player.as_str();
        println!("opponent: {}", opponent);
        println!("player: {}", player);

        match player {
            ROCK => {
                println!("player rock");
                score += ROCK_SCORE;
                match opponent {
                    ROCK_OPP => score += TIE_SCORE,
                    PAPER_OPP => score += LOSE_SCORE,
                    SCISSORS_OPP => score += WIN_SCORE,
                    _ => println!("Error in matching opponent with rock"),
                }
            }
            PAPER => {
                println!("player paper");
                score += PAPER_SCORE;
                match opponent {
                    ROCK_OPP => score += WIN_SCORE,
                    PAPER_OPP => score += TIE_SCORE,
                    SCISSORS_OPP => score += LOSE_SCORE,
                    _ => println!("Error in matching opponent with paper"),
                }
            }
            SCISSORS => {
                println!("player scissors");
                score += SCISSORS_SCORE;
                match opponent {
                    ROCK_OPP => score += LOSE_SCORE,
                    PAPER_OPP => score += WIN_SCORE,
                    SCISSORS_OPP => score += TIE_SCORE,
                    _ => println!("Error in matching opponent with scissors"),
                }
            }
            _ => {
                println!("Invalid input");
            }
        }
    }

    // part 2  
    for line in input.lines() {
        let opponent = line[0..1].to_string();
        let result = line[2..3].to_string();
        let opponent = opponent.as_str();
        let result = result.as_str();
        println!("opponent: {}", opponent);
        println!("player: {}", result);
        match result {
            LOSE => {
                scorept2 += 0;
                println!("player lost");
                match opponent { // add score based on what is needed to lose
                    ROCK_OPP => scorept2 += SCISSORS_SCORE,
                    PAPER_OPP => scorept2 += ROCK_SCORE,
                    SCISSORS_OPP => scorept2 += PAPER_SCORE,
                    _ => println!("Error in matching opponent with loss"),
                }
            }
            TIE => {
                scorept2 += 3;
                println!("player tied");
                match opponent {
                    ROCK_OPP => scorept2 += ROCK_SCORE,
                    PAPER_OPP => scorept2 += PAPER_SCORE,
                    SCISSORS_OPP => scorept2 += SCISSORS_SCORE,
                    _ => println!("Error in matching opponent with tie"),
                }
            }
            WIN => {
                scorept2 += 6;
                println!("player won");
                match opponent {
                    ROCK_OPP => scorept2 += PAPER_SCORE,
                    PAPER_OPP => scorept2 += SCISSORS_SCORE,
                    SCISSORS_OPP => scorept2 += ROCK_SCORE,
                    _ => println!("Error in matching opponent with win"),
                }
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
    
    (score, scorept2)
}