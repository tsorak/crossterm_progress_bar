mod progress_bar;
use progress_bar::ProgressBar;

fn main() -> anyhow::Result<()> {
    let mut max_value = 200;

    let mut bar = ProgressBar::new(max_value)
        .set_width(50)
        .set_show_percent(false);

    let mut i = 0;
    loop {
        bar.set_progress(i)?;

        if i == 50 {
            bar.with_width("stretch");
        }

        if i == 100 {
            bar.with_show_percent(true);
        }

        if i == 150 {
            max_value = 175;
            bar.with_max_value(max_value);
        }

        if i == 175 {
            break;
        }

        i += 1;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!();

    Ok(())
}
