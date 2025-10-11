pub fn get_card(suit: u8, card: u8) -> String {
    let suit_str = match suit {
        1 => "♠".to_string(),
        2 => "♥".to_string(),
        3 => "♣".to_string(),
        4 => "♦".to_string(),
        _ => "Error".to_string(),
    };

    let card_str = match card {
        0 => "".to_string(),
        1 => "Ace".to_string(),
        2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => card.to_string(),
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
        _ => "Error".to_string(),
    };

    format!("{} {}", card_str, suit_str)
}
