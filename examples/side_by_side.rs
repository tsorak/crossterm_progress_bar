use std::io::{stdout, Write};

use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    ExecutableCommand,
};
use crossterm_progress_bar::ProgressBar;

fn main() -> Result<(), std::io::Error> {
    let max_value = 100;

    let mut bar1 = ProgressBar::new(max_value)
        .set_width(50)
        .set_show_percent(false);

    bar1.style
        .with_fill(|ch| ch.red())
        .with_arrow(|ch| ch.red())
        .with_empty(|_| '-'.dim());

    bar1.render()?;

    let mut bar2 = bar1.clone();

    bar2.style
        .with_fill(|ch| ch.blue())
        .with_arrow(|ch| ch.blue());

    bar2.render()?;

    //

    let mut stdout = stdout();

    for i in 0..max_value + 1 {
        let l = bar1.set_progress(i).render_to_string()?;
        let r = bar2.set_progress(i).render_to_string()?;

        let formatted = format!("{l} {r}").stylize();

        stdout.execute(cursor::SavePosition)?;
        stdout.execute(PrintStyledContent(formatted))?;
        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!();

    Ok(())
}
