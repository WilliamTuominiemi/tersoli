use ratatui::{
    Frame,
    layout::Layout,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Widget, canvas::Canvas},
};

use crate::{
    card::Card, foundation::Foundation, location::Location, stock::Stock, suit::Suit,
    tableau::Tableau, utils::get_suit_by_card_suit_index, waste::Waste,
};

pub fn render(
    frame: &mut Frame,
    horizontal: Layout,
    vertical: Layout,
    tableau: &Tableau,
    stock: &Stock,
    waste: &Waste,
    foundation: &Foundation,
    selected: Location,
    active: Option<Location>,
    won: bool,
) {
    let [top, bottom] = vertical.areas(frame.area());
    let [
        stock_rect,
        waste_rect,
        second_empty,
        spades,
        hearts,
        clubs,
        diamonds,
    ] = horizontal.areas(top);

    let [first, second, third, fourth, fifth, sixth, seventh] = horizontal.areas(bottom);

    frame.render_widget(
        stock_canvas(Location::Stock, stock, selected, active),
        stock_rect,
    );
    frame.render_widget(
        waste_canvas(Location::Waste, waste, selected, active),
        waste_rect,
    );
    frame.render_widget(empty_canvas(won), second_empty);
    frame.render_widget(
        foundation_canvas(Location::Foundation(0), foundation, selected, active),
        spades,
    );
    frame.render_widget(
        foundation_canvas(Location::Foundation(1), foundation, selected, active),
        hearts,
    );
    frame.render_widget(
        foundation_canvas(Location::Foundation(2), foundation, selected, active),
        clubs,
    );
    frame.render_widget(
        foundation_canvas(Location::Foundation(3), foundation, selected, active),
        diamonds,
    );

    frame.render_widget(
        card_canvas(Location::Tableau(0), tableau, selected, active),
        first,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(1), tableau, selected, active),
        second,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(2), tableau, selected, active),
        third,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(3), tableau, selected, active),
        fourth,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(4), tableau, selected, active),
        fifth,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(5), tableau, selected, active),
        sixth,
    );
    frame.render_widget(
        card_canvas(Location::Tableau(6), tableau, selected, active),
        seventh,
    );
}

fn card_canvas(
    pos: Location,
    tableau: &Tableau,
    selected: Location,
    active: Option<Location>,
) -> impl Widget {
    let (visible_cards, card_text) = match pos {
        Location::Tableau(index) => (
            tableau.get_visible_cards(index),
            format!("Hidden: {}", tableau.cutoffs[index]),
        ),
        _ => unreachable!("Can't draw tableau other than in tableau"),
    };

    Canvas::default()
        .block(
            Block::bordered()
                .title(card_text)
                .border_style(canvas_style(pos, selected, active)),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            ctx.layer();
            let amount_of_cards = visible_cards.len() as f64;

            for (i, card) in visible_cards.iter().enumerate() {
                let card_name = get_card(card.suit, card.rank);

                ctx.print(
                    10.0,
                    100.0 - (100.0 / (amount_of_cards) * i as f64),
                    Span::styled(card_name.to_string(), card_text_style(Some(*card))),
                );
            }
        })
}

fn stock_canvas(
    pos: Location,
    stock: &Stock,
    selected: Location,
    active: Option<Location>,
) -> impl Widget {
    let card_amount = stock.cards.len();

    let card_text = format!("In stock: {}", card_amount);

    Canvas::default()
        .block(
            Block::bordered()
                .title(card_text)
                .border_style(canvas_style(pos, selected, active)),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|_ctx| {})
}

fn waste_canvas(
    pos: Location,
    waste: &Waste,
    selected: Location,
    active: Option<Location>,
) -> impl Widget {
    let cards = waste.get_last_cards();

    Canvas::default()
        .block(
            Block::bordered()
                .title("Waste pile")
                .border_style(canvas_style(pos, selected, active)),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            ctx.layer();
            if cards.is_empty() {
                ctx.print(10.0, 50.0, Span::styled("Empty", card_text_style(None)));
            } else {
                let amount_of_cards = cards.len() as f64;
                for (i, card) in cards.iter().enumerate() {
                    let card_name = get_card(card.suit, card.rank);
                    ctx.print(
                        10.0,
                        100.0 - (100.0 / (amount_of_cards) * i as f64),
                        Span::styled(card_name.to_string(), card_text_style(Some(*card))),
                    );
                }
            }
        })
}

fn empty_canvas(won: bool) -> impl Widget {
    Canvas::default()
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            if won {
                ctx.layer();
                ctx.print(
                    10.0,
                    50.0,
                    Span::styled("You win!", Style::default().fg(Color::Magenta)),
                );
            }
        })
}

fn foundation_canvas(
    pos: Location,
    foundation: &Foundation,
    selected: Location,
    active: Option<Location>,
) -> impl Widget {
    let suit_index = match pos {
        Location::Foundation(index) => index,
        _ => unreachable!("Can't draw foundation other than in foundation"),
    };

    let card_name = get_card(
        get_suit_by_card_suit_index(suit_index),
        foundation.get_top_value(pos),
    );

    Canvas::default()
        .block(
            Block::bordered()
                .title(match suit_index {
                    0 => "Spades",
                    1 => "Hearts",
                    2 => "Clubs",
                    3 => "Diamonds",
                    _ => "Error",
                })
                .border_style(canvas_style(pos, selected, active)),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            ctx.layer();
            ctx.print(
                10.0,
                50.0,
                Span::styled(
                    card_name.to_string(),
                    Style::default().fg(if (suit_index + 1) % 2 == 0 {
                        Color::LightRed
                    } else {
                        Color::LightGreen
                    }),
                ),
            )
        })
}

fn get_card(suit: Suit, rank: u8) -> String {
    let suit_str = match suit {
        Suit::Spades => "♠".to_string(),
        Suit::Hearts => "♥".to_string(),
        Suit::Clubs => "♣".to_string(),
        Suit::Diamonds => "♦".to_string(),
    };

    let rank_str = match rank {
        0 => "".to_string(),
        1 => "Ace".to_string(),
        2..=10 => rank.to_string(),
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
        _ => "Error".to_string(),
    };

    format!("{} {}", rank_str, suit_str)
}

fn canvas_style(pos: Location, selected: Location, active: Option<Location>) -> Style {
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

fn card_text_style(card: Option<Card>) -> Style {
    Style::default().fg(match card {
        Some(c) => match c.suit {
            Suit::Hearts | Suit::Diamonds => Color::LightRed,
            Suit::Clubs | Suit::Spades => Color::LightGreen,
        },
        _ => Color::Yellow,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card() {
        assert_eq!(get_card(Suit::Hearts, 8), "8 ♥");
        assert_eq!(get_card(Suit::Spades, 13), "King ♠");
    }

    #[test]
    fn test_canvas_style() {
        assert_eq!(
            canvas_style(Location::Stock, Location::Stock, Some(Location::Stock)),
            Style::default().fg(Color::Green)
        );
        assert_eq!(
            canvas_style(Location::Stock, Location::Stock, None),
            Style::default().fg(Color::Blue)
        );
        assert_eq!(
            canvas_style(
                Location::Tableau(1),
                Location::Stock,
                Some(Location::Tableau(1))
            ),
            Style::default().fg(Color::Red)
        );
        assert_eq!(
            canvas_style(
                Location::Stock,
                Location::Foundation(0),
                Some(Location::Tableau(1))
            ),
            Style::default().fg(Color::White)
        );
    }

    #[test]
    fn test_card_text_style() {
        let card1 = Card::new(Suit::Clubs, 1);
        let card2 = Card::new(Suit::Hearts, 2);

        assert_eq!(
            card_text_style(Some(card1)),
            Style::default().fg(Color::LightGreen)
        );
        assert_eq!(
            card_text_style(Some(card2)),
            Style::default().fg(Color::LightRed)
        );
    }
}
