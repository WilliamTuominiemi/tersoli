use ratatui::{
    Frame,
    layout::Layout,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Widget, canvas::Canvas},
};

use crate::{
    foundation::Foundation,
    stock::Stock,
    tableau::Tableau,
    utils::{canvas_style, card_text_style, get_card},
    waste::Waste,
};

use crate::card::Card;

pub fn render(
    frame: &mut Frame,
    horizontal: Layout,
    vertical: Layout,
    tableau: &Tableau,
    stock: &Stock,
    waste: &Waste,
    foundation: &Foundation,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
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

    frame.render_widget(stock_canvas((0, 0), &stock, selected, active), stock_rect);
    frame.render_widget(waste_canvas((1, 0), &waste, selected, active), waste_rect);
    frame.render_widget(empty_canvas(), second_empty);
    frame.render_widget(
        foundation_canvas((3, 0), &foundation, selected, active),
        spades,
    );
    frame.render_widget(
        foundation_canvas((4, 0), &foundation, selected, active),
        hearts,
    );
    frame.render_widget(
        foundation_canvas((5, 0), &foundation, selected, active),
        clubs,
    );
    frame.render_widget(
        foundation_canvas((6, 0), &foundation, selected, active),
        diamonds,
    );

    frame.render_widget(card_canvas((0, 1), &tableau, selected, active), first);
    frame.render_widget(card_canvas((1, 1), &tableau, selected, active), second);
    frame.render_widget(card_canvas((2, 1), &tableau, selected, active), third);
    frame.render_widget(card_canvas((3, 1), &tableau, selected, active), fourth);
    frame.render_widget(card_canvas((4, 1), &tableau, selected, active), fifth);
    frame.render_widget(card_canvas((5, 1), &tableau, selected, active), sixth);
    frame.render_widget(card_canvas((6, 1), &tableau, selected, active), seventh);
}

pub fn card_canvas(
    pos: (i8, i8),
    tableau: &Tableau,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
) -> impl Widget {
    let visible_cards: Vec<Card> = tableau.get_visible_cards(pos.0);

    let card_text = format!("Hidden: {}", tableau.cutoffs[pos.0 as usize]);

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
                    Span::styled(format!("{}", card_name), card_text_style(Some(*card))),
                );
            }
        })
}

pub fn stock_canvas(
    pos: (i8, i8),
    stock: &Stock,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
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

pub fn waste_canvas(
    pos: (i8, i8),
    waste: &Waste,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
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
                        Span::styled(format!("{}", card_name), card_text_style(Some(*card))),
                    );
                }
            }
        })
}

pub fn empty_canvas() -> impl Widget {
    Canvas::default().paint(|_ctx| {})
}

pub fn foundation_canvas(
    pos: (i8, i8),
    foundation: &Foundation,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
) -> impl Widget {
    let suit_index = pos.0 - 3;

    let card_name = get_card((suit_index + 1) as u8, foundation.get_top_value(pos));

    Canvas::default()
        .block(
            Block::bordered()
                .title("Foundation")
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
                    format!("{}", card_name),
                    Style::default().fg(if pos.0 % 2 == 0 {
                        Color::LightRed
                    } else {
                        Color::LightGreen
                    }),
                ),
            )
        })
}
