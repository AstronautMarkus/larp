use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rand::RngExt;
use std::thread::sleep;
use std::time::Duration;

pub fn line(text: &str) {
    println!("{}", styled_line(text));
    jitter_sleep(35, 140);
}

pub fn blank() {
    println!();
}

fn styled_line(text: &str) -> String {
    if text.starts_with('[') {
        if let Some(end) = text.find(']') {
            let tag = &text[..=end];
            let rest = &text[end + 1..];
            let styled_tag = match tag {
                "[larp]" => style(tag).cyan().bold().to_string(),
                "[\u{2713}]" => style(tag).green().bold().to_string(),
                "[!]" | "[x]" => style(tag).yellow().bold().to_string(),
                _ => style(tag).blue().bold().to_string(),
            };
            return format!("{styled_tag}{rest}");
        }
    }
    text.to_string()
}

pub fn spinner(msg: &str, min_ms: u64, max_ms: u64) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    jitter_sleep(min_ms, max_ms);
    pb.finish_and_clear();
    println!("{} {}", style("[\u{2713}]").green().bold(), msg);
}

pub fn progress_bar(label: &str) {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::with_template("{prefix} [{bar:24.cyan/blue}] {percent}%")
            .unwrap()
            .progress_chars("█▓░"),
    );
    pb.set_prefix(label.to_string());
    for _ in 0..100 {
        pb.inc(1);
        sleep(Duration::from_millis(rand_range(3, 10)));
    }
    pb.finish();
}

pub fn jitter_sleep(min_ms: u64, max_ms: u64) {
    sleep(Duration::from_millis(rand_range(min_ms, max_ms)));
}

pub fn rand_range(min: u64, max: u64) -> u64 {
    if min >= max {
        return min;
    }
    rand::rng().random_range(min..=max)
}
