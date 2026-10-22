// Installations de Discord : où elles sont, dans quel état, et comment les
// fermer puis les relancer.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

use crate::asar;
use crate::paths::Paths;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Branch {
    Stable,
    Ptb,
    Canary,
    Development,
}

impl Branch {
    pub const ALL: [Branch; 4] = [Branch::Stable, Branch::Ptb, Branch::Canary, Branch::Development];

    fn folder(self) -> &'static str {
        match self {
            Branch::Stable => "Discord",
            Branch::Ptb => "DiscordPTB",
            Branch::Canary => "DiscordCanary",
            Branch::Development => "DiscordDevelopment",
        }
    }

    fn exe(self) -> &'static str {
        match self {
            Branch::Stable => "Discord.exe",
            Branch::Ptb => "DiscordPTB.exe",
            Branch::Canary => "DiscordCanary.exe",
            Branch::Development => "DiscordDevelopment.exe",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Branch::Stable => "Discord",
            Branch::Ptb => "Discord PTB",
            Branch::Canary => "Discord Canary",
            Branch::Development => "Discord Development",
        }
    }

    pub fn root(self, paths: &Paths) -> PathBuf {
        paths.local_appdata.join(self.folder())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// Discord d'origine
    Original,
    /// Branché sur les fichiers de Strata
    Strata,
    /// Branché sur autre chose : Vencord officiel, installation de développement…
    Other,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Install {
    pub branch: Branch,
    pub label: &'static str,
    pub version: String,
    pub resources: PathBuf,
    pub state: State,
    /// patcher.js chargé quand Discord est modifié
    pub patcher: Option<String>,
    pub running: bool,
}

// « app-1.0.9264 » → [1, 0, 9264], pour comparer les versions numériquement
fn version_of(dir: &OsStr) -> Option<Vec<u64>> {
    dir.to_str()?
        .strip_prefix("app-")?
        .split('.')
        .map(|n| n.parse().ok())
        .collect()
}

/// Dossier de la version la plus récente (Discord garde parfois l'ancienne à côté)
pub fn latest_app_dir(root: &Path) -> Option<PathBuf> {
    fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| e.path().join("resources").is_dir())
        .filter_map(|e| Some((version_of(&e.file_name())?, e.path())))
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, path)| path)
}

fn same_file(a: &str, b: &Path) -> bool {
    a.replace('/', "\\").eq_ignore_ascii_case(&b.to_string_lossy())
}

pub fn state_of(resources: &Path, paths: &Paths) -> (State, Option<String>) {
    if !resources.join("_app.asar").exists() {
        return (State::Original, None);
    }
    let patcher = fs::read(resources.join("app.asar"))
        .ok()
        .and_then(|bytes| asar::patcher_of(&bytes));
    let state = match &patcher {
        Some(p) if same_file(p, &paths.patcher()) => State::Strata,
        _ => State::Other,
    };
    (state, patcher)
}

pub fn find(paths: &Paths) -> Vec<Install> {
    let running = running_exes();
    Branch::ALL
        .into_iter()
        .filter_map(|branch| {
            let app_dir = latest_app_dir(&branch.root(paths))?;
            let resources = app_dir.join("resources");
            let (state, patcher) = state_of(&resources, paths);
            Some(Install {
                branch,
                label: branch.label(),
                version: app_dir
                    .file_name()?
                    .to_string_lossy()
                    .trim_start_matches("app-")
                    .to_string(),
                resources,
                state,
                patcher,
                running: running.iter().any(|exe| exe.eq_ignore_ascii_case(branch.exe())),
            })
        })
        .collect()
}

fn processes() -> System {
    let mut system = System::new_with_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()));
    system.refresh_processes(ProcessesToUpdate::All, true);
    system
}

fn running_exes() -> Vec<String> {
    processes()
        .processes()
        .values()
        .map(|p| p.name().to_string_lossy().into_owned())
        .collect()
}

/// Ferme Discord (tous ses processus) et attend qu'il ait libéré ses fichiers
pub fn close(branch: Branch) -> Result<(), String> {
    let exe = OsStr::new(branch.exe());
    let system = processes();
    for process in system.processes_by_exact_name(exe) {
        process.kill();
    }

    let deadline = Instant::now() + Duration::from_secs(10);
    while processes().processes_by_exact_name(exe).next().is_some() {
        if Instant::now() > deadline {
            return Err(format!(
                "{} ne s'est pas fermé. Ferme-le à la main puis réessaie.",
                branch.label()
            ));
        }
        sleep(Duration::from_millis(250));
    }
    // Windows libère les fichiers ouverts juste après la fin des processus
    sleep(Duration::from_millis(500));
    Ok(())
}

/// Relance Discord comme son raccourci : Update.exe --processStart Discord.exe
pub fn launch(branch: Branch, paths: &Paths) -> Result<(), String> {
    Command::new(branch.root(paths).join("Update.exe"))
        .args(["--processStart", branch.exe()])
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Impossible de relancer {} : {e}", branch.label()))
}
