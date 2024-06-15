use ratatui::{prelude::*, widgets::*};
use crate::tui_app;


pub fn build_ui(frame: &mut Frame, visual_data: Vec<u64>) {

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    ).split(frame.size());


    frame.render_widget(
        Block::new().magenta().borders(Borders::TOP).title("relaks").title_alignment(Alignment::Center),
        main_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ],
    ).split(main_layout[1]);

    let left = Layout::new(
        Direction::Vertical,
        [
            Constraint::Max(10),
            Constraint::Percentage(90),
            Constraint::Max(50),
        ]
    ).split(inner_layout[0]);


    frame.render_widget(
        Paragraph::new(format!("Music playing now: {}", tui_app::CURRENT_TRACK.lock().unwrap())).magenta().bold().italic().alignment(Alignment::Center).block(Block::bordered().title("Title")),
        left[0],
    );
    
    
    let bottom_left = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(100)
        ]
    ).split(left[1]);
    

    frame.render_widget(Paragraph::new(format!("{}", tui_app::DESC.lock().unwrap())).wrap( Wrap { trim: true }).centered().block(Block::bordered().magenta().title("Description").bold().italic()), bottom_left[0]);
    
    let right = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(100),
        ]
    ).split(inner_layout[1]);

    frame.render_widget(Block::default(), right[0]);

    
    let barchart = BarChart::default()
        .block(Block::default().title("Visual").bold().borders(Borders::ALL).padding(Padding::horizontal(10)))
        .bar_width(15)
        .bar_gap(5)
        .bar_style(Style::default().fg(Color::LightMagenta))
        .data(
            BarGroup::default().bars(   
                &[
                    Bar::default().value(visual_data[0]), 
                    Bar::default().value(visual_data[1]),
                    Bar::default().value(visual_data[2]),
                    Bar::default().value(visual_data[3]),
                    Bar::default().value(visual_data[4]),
                    Bar::default().value(visual_data[5]),
                ]
            ));
    

    

    frame.render_widget(barchart.magenta(), right[0]);
    
}