use crossterm::style::Stylize;
use crossterm_progress_bar::ProgressBar;

fn main() -> anyhow::Result<()> {
    let mut max_value = 250;

    let mut bar = ProgressBar::new(max_value)
        .set_width(50)
        .set_show_percent(false);

    bar.render()?;
    print_under("Absolute width progress bar")?;

    bar.style
        .with_fill(|ch| ch.green())
        .with_arrow(|ch| ch.green())
        .with_empty(|_| '-'.dim());

    let mut i = 0;
    loop {
        bar.set_progress(i).render()?;

        if i == 50 {
            print_under("Span progress bar across screen")?;
            bar.with_width("stretch");
        }

        if i == 100 {
            print_under("Show percentage")?;
            bar.with_show_percent(true);
        }

        if i == 150 {
            print_under("Change max value midway through")?;
            max_value = 200;
            bar.with_max_value(max_value);
        }

        if i == 200 {
            break;
        }

        i += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!();

    Ok(())
}

fn print_under(s: &str) -> anyhow::Result<()> {
    use crossterm::{
        cursor::{MoveDown, RestorePosition, SavePosition},
        style::PrintStyledContent,
        style::Stylize,
        terminal, ExecutableCommand,
    };
    use std::io::{stdout, Write};

    let mut stdout = stdout();

    stdout.execute(SavePosition)?;
    stdout.execute(MoveDown(1))?;
    stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
    stdout.execute(PrintStyledContent(s.to_string().stylize()))?;
    stdout.execute(RestorePosition)?;
    stdout.flush()?;

    Ok(())
}
