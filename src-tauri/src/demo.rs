// Mode démo (variable STRATA_DEMO=1) : l'interface montre deux Discord simulés et
// chaque opération déroule ses étapes à l'écran, sans toucher au vrai Discord ni
// au réseau. Sert à essayer l'interface en toute sécurité.

use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use crate::discord::{Branch, Install, State};
use crate::release::Release;

pub fn enabled() -> bool {
    std::env::var_os("STRATA_DEMO").is_some_and(|v| v == "1")
}

static STATES: Mutex<[(Branch, State); 2]> =
    Mutex::new([(Branch::Stable, State::Original), (Branch::Ptb, State::Strata)]);

pub fn installs() -> Vec<Install> {
    STATES
        .lock()
        .unwrap()
        .iter()
        .map(|&(branch, state)| Install {
            branch,
            label: branch.label(),
            version: if branch == Branch::Stable {
                "1.0.9264".into()
            } else {
                "1.0.1182".into()
            },
            resources: Default::default(),
            state,
            patcher: None,
            running: branch == Branch::Stable,
        })
        .collect()
}

pub fn release() -> Release {
    Release {
        name: "Strata 70860a5".into(),
        hash: "70860a5".into(),
    }
}

fn set_state(branch: Branch, state: State) {
    for entry in STATES.lock().unwrap().iter_mut().filter(|e| e.0 == branch) {
        entry.1 = state;
    }
}

/// Déroule les étapes comme le vrai launcher, avec des pauses réalistes
pub fn run(steps: &[&str], step: impl Fn(&str), branch: Branch, state: State) {
    for message in steps {
        step(message);
        sleep(Duration::from_millis(900));
    }
    set_state(branch, state);
}
