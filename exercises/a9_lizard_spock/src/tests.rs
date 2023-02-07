use crate::{Game, Winner};

#[test]
fn check_a_single_match() {
    let mut game = Game::new();
    let test_rounds_list = [["🖖", "🗿"]]
        .iter()
        .map(|pair| vec![pair[0].to_string(), pair[1].to_string()])
        .collect::<Vec<Vec<String>>>();
    game.check_all_rounds(test_rounds_list);
    assert_eq!(Winner::Player1, game.get_winner());
}

#[test]
fn check_player2_wins() {
    //  Ejemplo. Entrada: [("🗿","✂️"), ("✂️","🗿"), ("📄","✂️")]. Resultado: "Player 2".
    let mut game = Game::new();
    let test_rounds_list = [["🗿", "✂️"], ["✂️", "🗿"], ["📄", "✂️"]]
        .iter()
        .map(|pair| vec![pair[0].to_string(), pair[1].to_string()])
        .collect::<Vec<Vec<String>>>();
    game.check_all_rounds(test_rounds_list);
    assert_eq!(Winner::Player2, game.get_winner());
}

#[test]
fn check_draw() {
    let mut game = Game::new();
    let test_rounds_list = [["🦎", "🖖"], ["🗿", "📄"]]
        .iter()
        .map(|pair| vec![pair[0].to_string(), pair[1].to_string()])
        .collect::<Vec<Vec<String>>>();
    game.check_all_rounds(test_rounds_list);
    assert_eq!(Winner::Draw, game.get_winner());
}

#[test]
fn check_player1_wins() {
    let mut game = Game::new();
    let test_rounds_list = [
        ["🦎", "🖖"],
        ["🗿", "✂️"],
        ["🗿", "📄"],
        ["🖖", "✂️"],
        ["🖖", "📄"],
    ]
    .iter()
    .map(|pair| vec![pair[0].to_string(), pair[1].to_string()])
    .collect::<Vec<Vec<String>>>();
    game.check_all_rounds(test_rounds_list);
    assert_eq!(Winner::Player1, game.get_winner());
}
