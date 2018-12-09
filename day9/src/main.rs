use std::env;

struct Game {
    circle : Vec<usize>,
    cur_marble : usize,
    scores : Vec<usize>,
    cur_player : usize,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let players = args[1].parse::<usize>().unwrap();
    let marbles = args[2].parse::<usize>().unwrap();
    println!("{} players, {} marbles", players, marbles);
    let game = play_game(players, marbles);
    let max_score = game.scores.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap();
    println!("Max score {} for player {}", max_score.0, max_score.1);
}

fn play_game(players : usize, marbles : usize) -> Game {
    let mut game = Game{circle : vec![0;1], cur_marble: 0, scores : vec![0;players], cur_player : 0,};
    for i in 1..=marbles {
        //println!("{:?}", game.circle);
        if i % 23 != 0 {
            let mut insert_point = game.cur_marble + 2;
            if insert_point == game.circle.len() {
                game.circle.push(i);
            }
            else {
                if insert_point > game.circle.len() {
                    insert_point -= game.circle.len()
                }
                game.circle.insert(insert_point, i);
            }

            println!("player {} inserts {}@{}", game.cur_player, i, insert_point);
            game.cur_marble = insert_point;
        }
        else {
            game.cur_marble = if game.cur_marble >= 7 {
                game.cur_marble - 7
            }
            else {
                game.cur_marble + game.circle.len() - 7
            };
            game.scores[game.cur_player] += i;
            let removed_marble = game.circle.remove(game.cur_marble);
            println!("player {} removes {}@{}", game.cur_player, removed_marble, game.cur_marble);
            game.scores[game.cur_player] += removed_marble;
            if game.cur_marble == game.circle.len() {
                game.cur_marble = 0;
            }
        }
        game.cur_player += 1;
        game.cur_player %= players;
    }
    game
}
