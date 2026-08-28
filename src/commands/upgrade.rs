use crate::scripts::{self, packages};
use crate::state::AppState;
use crate::theatrics;

pub fn run(state: &mut AppState, package: Option<&str>) {
    match package {
        Some(pkg) => upgrade_one(state, pkg),
        None => {
            if state.installed.is_empty() {
                theatrics::line("[larp] Nothing to upgrade. The void remains empty.");
                return;
            }
            let keys: Vec<String> = state.installed.keys().cloned().collect();
            for key in keys {
                upgrade_one(state, &key);
            }
        }
    }
}

fn upgrade_one(state: &mut AppState, package: &str) {
    let key = package.to_lowercase();
    let from = match state.installed.get(&key) {
        Some(v) => v.clone(),
        None => {
            theatrics::line(&format!(
                "[larp] {package} wasn't installed. Installing it first, for your convenience."
            ));
            let v = packages::random_version();
            state.installed.insert(key.clone(), v.clone());
            v
        }
    };
    let to = packages::bump_version(&from);
    let script = packages::upgrade_script(package, &from, &to);
    scripts::run(&script);
    state.installed.insert(key, to);
}
