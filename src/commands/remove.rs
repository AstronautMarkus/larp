use crate::scripts::{self, packages};
use crate::state::AppState;

pub fn run(state: &mut AppState, package: &str) {
    let key = package.to_lowercase();
    let was_installed = state.installed.remove(&key).is_some();
    let script = packages::remove_script(package, was_installed);
    scripts::run(&script);
}
