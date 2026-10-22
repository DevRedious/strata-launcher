mod asar;
mod demo;
mod discord;
mod patch;
mod paths;
mod release;

use discord::{Branch, Install, State};
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

fn find_install(paths: &Paths, branch: Branch) -> Result<Install, String> {
    discord::find(paths)
        .into_iter()
        .find(|i| i.branch == branch)
        .ok_or_else(|| format!("{} n'est pas installé.", branch.label()))
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
                "Téléchargement de Strata…",
                &format!("Fermeture de {}…", branch.label()),
                "Installation…",
                &format!("Relance de {}…", branch.label()),
            ];
            demo::run(&steps, |m| step(&app, m), branch, State::Strata);
            Ok(format!(
                "Strata 70860a5 installé sur {} (démo : rien n'a été modifié).",
                branch.label()
            ))
        })
        .await;
    }
    blocking(move || {
        let paths = Paths::from_env()?;
        let target = find_install(&paths, branch)?;

        // Téléchargement d'abord, Discord encore ouvert : il n'est fermé que le
        // temps de modifier un fichier
        step(&app, "Téléchargement de Strata…");
        let release = release::download(&paths.dist())?;

        step(&app, &format!("Fermeture de {}…", branch.label()));
        discord::close(branch)?;

        step(&app, "Installation…");
        patch::apply(&target.resources, &paths.patcher())?;

        step(&app, &format!("Relance de {}…", branch.label()));
        discord::launch(branch, &paths)?;

        Ok(format!("{} installé sur {}.", release.name, branch.label()))
    })
    .await
}

#[tauri::command]
async fn uninstall(app: AppHandle, branch: Branch) -> Result<String, String> {
    if demo::enabled() {
        return blocking(move || {
            let steps = [
                &format!("Fermeture de {}…", branch.label()),
                "Désinstallation…",
                &format!("Relance de {}…", branch.label()),
            ];
            demo::run(&steps, |m| step(&app, m), branch, State::Original);
            Ok(format!(
                "Strata a été retiré de {} (démo : rien n'a été modifié).",
                branch.label()
            ))
        })
        .await;
    }
    blocking(move || {
        let paths = Paths::from_env()?;
        let target = find_install(&paths, branch)?;

        step(&app, &format!("Fermeture de {}…", branch.label()));
        discord::close(branch)?;

        step(&app, "Désinstallation…");
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

        step(&app, &format!("Relance de {}…", branch.label()));
        discord::launch(branch, &paths)?;

        Ok(format!("Strata a été retiré de {}.", branch.label()))
    })
    .await
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![installs, latest_release, install, uninstall])
        .run(tauri::generate_context!())
        .expect("erreur au démarrage du launcher");
}
