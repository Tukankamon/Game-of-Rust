
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{stdout},
    thread::sleep,
    time::Duration,
};
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    
    #[arg(short, long, default_value_t = Color::Green)]
    color: Color,

    #[arg(short, long, default_value_t = 10)]
    rate: u64,

    #[arg(short, long, default_value_t = 0)]  // seed 1 is random
    seed: u64,

    #[arg(short, long, default_value_t = 1)]
    preset: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
  
    let args = Args::parse();

    let (width, height) = {  //Doesnt dynamically update
        let (w, h) = crossterm::terminal::size()?;
        (w as usize, h as usize)
    };
    
    let mut grid = vec![vec![false; width]; height];

    /*
    // Glider pattern
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;
    */

    grid[10][10] = true;
    grid[10][11] = true;
    grid[10][12] = true;

    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(0)].as_ref())
                .split(f.size());

            let cells: String = grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&cell| if cell { "██" } else { "  " })  // IDk how to make it more square
                        .collect::<String>()
                        + "\n"
                })
                .collect();

            let block = Paragraph::new(cells)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(args.color));  //Change color here

            f.render_widget(block, chunks[0]);
        })?;

        grid = step(grid, width, height);

        let time: u64 = 1000/args.rate;
        sleep(Duration::from_millis(time));
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn step(grid: Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let mut neighbors = 0;

            // Neihbours are like:
            // (-1, -1) (-1, 0) (-1, 1)
            // (0, -1)     x    (0, 1)
            // (1, -1) (1, 0)   (1, 1)
            
            for dy in [-1i32, 0, 1] {
                for dx in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let ny = (y as i32 + dy).rem_euclid(height as i32) as usize;  //rem is % but always positive
                    let nx = (x as i32 + dx).rem_euclid(width as i32) as usize;
                    if grid[ny][nx] {
                        neighbors += 1;
                    }
                }
            }

            new_grid[y][x] = matches!((grid[y][x], neighbors), (true, 2 | 3) | (false, 3));
            // basically the dead or alive logic
        }
    }

    new_grid
}
