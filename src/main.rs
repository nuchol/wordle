use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::{Block, Paragraph, Widget}};

enum Square {
    Green   (char),
    Yellow  (char),
    Empty   (char),
    Hovered (char),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let block = Block::bordered();
    let wordle_area = centered_rect(50, 100, frame.area());
    frame.render_widget(&block, wordle_area);
    render_wordle(block.inner(wordle_area), frame.buffer_mut());

}

fn render_wordle(area: Rect, buf: &mut Buffer) {
    for i in 0..5 {
        let square = Square::Empty('x');
        let size = area.width / 5;
        let area = Rect::new(
            area.x + size * i,
            area.y, size, size / 2);
        render_square(square, area, buf);
    }
}

fn render_square(square: Square, area: Rect, buf: &mut Buffer) {
    let r = match square {
        Square::Empty(c) => Paragraph::new(c.to_string()).block(Block::bordered()),
        _ => Paragraph::new(""),
    };

    r.render(area, buf);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let width = (area.width * percent_x) / 100;
    let height = (area.height * percent_y) / 100;
    let x = area.x + (area.width - width) / 2;
    let y = area.y + (area.height - height) / 2;

    Rect { x, y, width, height }
}
