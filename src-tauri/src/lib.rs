mod asar;
mod demo;
mod discord;
mod i18n;
mod icon;
mod patch;
mod paths;
mod release;

use discord::{Branch, Install, State};
use i18n::tr;
use paths::Paths;
use tauri::{AppHandle, Emitter};

// Les opérations bloquent (fichiers, réseau, attente de fermeture de Discord) :
// on les sort du fil de l'interface
async fn blocking<T: Send + 'static>(job: impl FnOnce() -> Result<T, String> + Send + 'static) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(job)
        .await
        .map_err(|e| e.to_string())?
}

// Étape en cours, affichée par l'interface
fn step(app: &AppHandle, message: &str) {
    let _ = app.emit("progress", message);
}

fn downloading() -> String {
    tr("Téléchargement de Strata…", "Downloading Strata…")
}

fn closing(branch: Branch) -> String {
    tr(
        &format!("Fermeture de {}…", branch.label()),
        &format!("Closing {}…", branch.label()),
    )
}

fn relaunching(branch: Branch) -> String {
    tr(
        &format!("Relance de {}…", branch.label()),
        &format!("Restarting {}…", branch.label()),
    )
}

fn find_install(paths: &Paths, branch: Branch) -> Result<Install, String> {
    discord::find(paths)
        .into_iter()
        .find(|i| i.branch == branch)
        .ok_or_else(|| {
            tr(
                &format!("{} n'est pas installé.", branch.label()),
                &format!("{} is not installed.", branch.label()),
            )
        })
}

#[tauri::command]
fn set_language(lang: String) {
    i18n::set(&lang);
}

#[tauri::command]
async fn installs() -> Result<Vec<Install>, String> {
    if demo::enabled() {
        return Ok(demo::installs());
    }
    blocking(|| Ok(discord::find(&Paths::from_env()?))).await
}

#[tauri::command]
async fn latest_release() -> Result<release::Release, String> {
    if demo::enabled() {
        return Ok(demo::release());
    }
    blocking(release::latest).await
}

#[tauri::command]
async fn install(app: AppHandle, branch: Branch) -> Result<String, String> {
    if demo::enabled() {
        return blocking(move || {
            let steps = [
                downloading(),
                closing(branch),
                tr("Installation…", "Installing…"),
                relaunching(branch),
            ];
            demo::run(
                &steps.each_ref().map(String::as_str),
                |m| step(&app, m),
                branch,
                State::Strata,
            );
            Ok(tr(
                &format!(
                    "Strata 70860a5 installé sur {} (démo : rien n'a été modifié).",
                    branch.label()
                ),
                &format!(
                    "Strata 70860a5 installed on {} (demo: nothing was changed).",
                    branch.label()
                ),
            ))
        })
        .await;
    }
    blocking(move || {
        let paths = Paths::from_env()?;
        let target = find_install(&paths, branch)?;

        // Téléchargement d'abord, Discord encore ouvert : il n'est fermé que le
        // temps de modifier un fichier
        step(&app, &downloading());
        let release = release::download(&paths.dist())?;

        step(&app, &closing(branch));
        discord::close(branch)?;

        step(&app, &tr("Installation…", "Installing…"));
        patch::apply(&target.resources, &paths.patcher())?;

        step(&app, &relaunching(branch));
        discord::launch(branch, &paths)?;

        Ok(tr(
            &format!("{} installé sur {}.", release.name, branch.label()),
            &format!("{} installed on {}.", release.name, branch.label()),
        ))
    })
    .await
}

#[tauri::command]
async fn uninstall(app: AppHandle, branch: Branch) -> Result<String, String> {
    if demo::enabled() {
        return blocking(move || {
            let steps = [
                closing(branch),
                tr("Désinstallation…", "Uninstalling…"),
                relaunching(branch),
            ];
            demo::run(
                &steps.each_ref().map(String::as_str),
                |m| step(&app, m),
                branch,
                State::Original,
            );
            Ok(tr(
                &format!(
                    "Strata a été retiré de {} (démo : rien n'a été modifié).",
                    branch.label()
                ),
                &format!(
                    "Strata was removed from {} (demo: nothing was changed).",
                    branch.label()
                ),
            ))
        })
        .await;
    }
    blocking(move || {
        let paths = Paths::from_env()?;
        let target = find_install(&paths, branch)?;

        step(&app, &closing(branch));
        discord::close(branch)?;

        step(&app, &tr("Désinstallation…", "Uninstalling…"));
        patch::remove(&target.resources)?;

        // Plus aucun Discord ne charge Strata : ses fichiers partent, les réglages
        // de Vencord (%APPDATA%\Vencord\settings) restent
        if !discord::find(&paths).iter().any(|i| i.state == State::Strata) {
            let _ = std::fs::remove_dir_all(paths.dist());
            // Puis le dossier Strata lui-même, s'il est vide (remove_dir refuse sinon)
            if let Some(strata) = paths.dist().parent() {
                let _ = std::fs::remove_dir(strata);
            }
        }

        step(&app, &relaunching(branch));
        discord::launch(branch, &paths)?;

        Ok(tr(
            &format!("Strata a été retiré de {}.", branch.label()),
            &format!("Strata was removed from {}.", branch.label()),
        ))
    })
    .await
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_language,
            installs,
            latest_release,
            install,
            uninstall
        ])
        .run(tauri::generate_context!())
        .expect("erreur au démarrage du launcher");
}
