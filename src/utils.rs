use ratatui::style::{Color, Style};

use crate::card::Card;

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

pub fn canvas_style(pos: (i8, i8), selected: (i8, i8), active: Option<(i8, i8)>) -> Style {
    let is_selected = pos == selected;
    let is_active = match active {
        Some(active) => pos == active,
        _ => false,
    };

    Style::default().fg(if is_selected && is_active {
        Color::Green
    } else if is_selected {
        Color::Blue
    } else if is_active {
        Color::Red
    } else {
        Color::White
    })
}

pub fn card_text_style(card: Option<Card>) -> Style {
    Style::default().fg(match card {
        Some(c) => {
            if c.suit % 2 == 0 {
                Color::Red
            } else {
                Color::Blue
            }
        }
        _ => Color::Yellow,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card() -> Result<(), String> {
        assert_eq!(get_card(3, 8), "8 ♣");
        assert_eq!(get_card(1, 13), "King ♠");
        Ok(())
    }

    #[test]
    fn test_canvas_style() -> Result<(), Style> {
        assert_eq!(
            canvas_style((0, 0), (0, 0), Some((0, 0))),
            Style::default().fg(Color::Green)
        );
        assert_eq!(
            canvas_style((0, 0), (0, 0), None),
            Style::default().fg(Color::Blue)
        );
        assert_eq!(
            canvas_style((1, 1), (0, 0), Some((1, 1))),
            Style::default().fg(Color::Red)
        );
        assert_eq!(
            canvas_style((0, 0), (2, 0), Some((1, 1))),
            Style::default().fg(Color::White)
        );
        Ok(())
    }

    #[test]
    fn test_card_text_style() -> Result<(), Style> {
        let card1 = Card::new(1, 1);
        let card2 = Card::new(2, 2);

        assert_eq!(
            card_text_style(Some(card1)),
            Style::default().fg(Color::Blue)
        );
        assert_eq!(
            card_text_style(Some(card2)),
            Style::default().fg(Color::Red)
        );
        Ok(())
    }
}
