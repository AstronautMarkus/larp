use crate::scripts::{self, packages};
use crate::state::AppState;

pub fn run(state: &mut AppState, package: &str) {
    let script = packages::install_script(package);
    scripts::run(&script);
    let version = packages::random_version();
    state.installed.insert(package.to_lowercase(), version);
}
