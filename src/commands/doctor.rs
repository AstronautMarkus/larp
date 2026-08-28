use crate::state::AppState;
use crate::theatrics;
use rand::RngExt;

pub fn run(state: &AppState) {
    theatrics::line("larp doctor");
    theatrics::line(&"-".repeat(32));

    if state.installed.is_empty() {
        theatrics::line("[larp] No packages installed. Suspiciously healthy.");
    } else {
        for (pkg, version) in &state.installed {
            theatrics::spinner(&format!("Checking {pkg} ({version})..."), 150, 400);
        }
    }

    theatrics::blank();
    let mut rng = rand::rng();
    let score = rng.random_range(87..=100);
    theatrics::line(&format!("System health: {score}% (also: subjective)"));

    let tips = [
        "Consider touching grass.",
        "Everything is fine. Probably.",
        "Recommendation: reboot your understanding of reality.",
        "No action needed. Or maybe all the action. Unclear.",
    ];
    let tip = tips[rng.random_range(0..tips.len())];
    theatrics::line(tip);
}
