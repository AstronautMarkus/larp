use super::Script;
use rand::RngExt;

pub fn install_script(package: &str) -> Script {
    match package.to_lowercase().as_str() {
        "nginx" => nginx(),
        "docker" => docker(),
        "arch" | "archlinux" | "arch-linux" | "arch linux" => arch(),
        "python" | "python3" => python(),
        _ => generic_install(package),
    }
}

fn nginx() -> Script {
    Script::new()
        .line("[larp] Resolving dependency graph...")
        .line("[larp] Checking kernel capabilities... OK")
        .spinner("Negotiating with systemd...", 400, 800)
        .line("[larp] Applying kernel-level optimizations...")
        .line("[larp] Compiling nginx from raw electrons...")
        .line("[larp] Installing nginx...")
        .progress("[larp]")
        .blank()
        .line("Successfully installed nginx.")
}

fn docker() -> Script {
    Script::new()
        .line("[+] Initializing container substrate")
        .line("[+] Mounting /dev/null as a strategic resource")
        .spinner("Asking systemd for permission...", 400, 700)
        .line("[!] systemd said no")
        .line("[+] Ignoring systemd")
        .line("[+] Rewriting reality...")
        .line("[\u{2713}] docker installed successfully")
        .blank()
        .line("Note: installation was performed entirely in your imagination.")
}

fn arch() -> Script {
    Script::new()
        .spinner("Downloading Arch Linux...", 500, 900)
        .line("[larp] Downloading 847 TB of packages...")
        .line("[larp] Actually, never mind.")
        .line("[larp] You use Debian.")
        .line("[larp] Installation complete.")
}

fn python() -> Script {
    let pct = rand::rng().random_range(12..=63);
    Script::new()
        .line("larp v0.1.0")
        .line("-".repeat(32))
        .line("Analyzing target system...")
        .line("Detecting CPU.................. AMD64")
        .line("Detecting kernel............... Linux")
        .line("Detecting user................. root")
        .line("Detecting skill level.......... suspiciously high")
        .blank()
        .spinner("Installing Python...", 500, 900)
        .line("  > optimizing interpreter")
        .line("  > removing unnecessary abstractions")
        .line("  > rewriting CPython in Rust")
        .line("  > questioning design decisions")
        .blank()
        .line("Done.")
        .blank()
        .line(format!("You are now {pct}% more employable."))
}

fn generic_install(package: &str) -> Script {
    let mut rng = rand::rng();
    let openers = [
        "Resolving dependency graph...",
        "Consulting the package oracle...",
        "Bribing the kernel scheduler...",
        "Warming up the compiler...",
    ];
    let actions = [
        "Compiling {pkg} from raw electrons...",
        "Downloading {pkg} from a mirror that only exists in your head...",
        "Cross-referencing {pkg} against 4 conflicting sources...",
        "Reticulating splines for {pkg}...",
    ];
    let flourishes = [
        "Applying kernel-level optimizations...",
        "Negotiating with systemd... OK",
        "Rewriting reality...",
        "Questioning several design decisions...",
    ];
    let closers = [
        "You are now marginally more powerful.",
        "This changes nothing, but it feels great.",
        "No refunds.",
        "It was never really necessary, but here we are.",
    ];

    let opener = openers[rng.random_range(0..openers.len())];
    let action = actions[rng.random_range(0..actions.len())].replace("{pkg}", package);
    let flourish = flourishes[rng.random_range(0..flourishes.len())];
    let closer = closers[rng.random_range(0..closers.len())];

    Script::new()
        .line(format!("[larp] {opener}"))
        .line(format!("[larp] {action}"))
        .line(format!("[larp] {flourish}"))
        .spinner(format!("Installing {package}..."), 500, 1000)
        .progress("[larp]")
        .blank()
        .line(format!("Successfully installed {package}."))
        .line(closer)
}

pub fn remove_script(package: &str, was_installed: bool) -> Script {
    if was_installed {
        Script::new()
            .line(format!("[larp] Locating {package} in the void..."))
            .spinner(format!("Un-compiling {package}..."), 400, 800)
            .line("[larp] Reversing the electrons...")
            .line(format!("[\u{2713}] Successfully removed {package}."))
            .line("It never truly existed.")
    } else {
        Script::new()
            .line(format!("[larp] Locating {package} in the void..."))
            .line(format!("[!] {package} is not installed."))
            .line("[larp] Removing it anyway, for closure.")
            .line(format!(
                "[\u{2713}] {package} has been removed from a system it was never on."
            ))
    }
}

pub fn upgrade_script(package: &str, from_version: &str, to_version: &str) -> Script {
    Script::new()
        .line(format!("[larp] Checking for updates to {package}..."))
        .line(format!(
            "[larp] Found 1 update: {package} {from_version} -> {to_version}"
        ))
        .spinner("Applying temporal patch...", 400, 800)
        .line(format!("[\u{2713}] Successfully upgraded {package}."))
}

pub fn random_version() -> String {
    let mut rng = rand::rng();
    format!(
        "{}.{}.{}-larp",
        rng.random_range(0..9),
        rng.random_range(0..20),
        rng.random_range(0..40)
    )
}

pub fn bump_version(v: &str) -> String {
    let base = v.split('-').next().unwrap_or(v);
    let mut parts: Vec<i64> = base.split('.').filter_map(|p| p.parse().ok()).collect();
    if let Some(last) = parts.last_mut() {
        *last += 1;
    } else {
        parts = vec![0, 0, 1];
    }
    let joined = parts
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(".");
    format!("{joined}-larp")
}
