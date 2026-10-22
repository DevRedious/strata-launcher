// Brancher Discord sur Strata, ou le rendre à son état d'origine. Discord doit
// être fermé : ce module ne touche ni aux processus ni au réseau.
//
// Discord charge resources/app.asar. On le renomme en _app.asar (Vencord charge
// ensuite ce Discord d'origine) et on écrit à sa place une petite archive qui
// appelle le patcher.js de Strata.

use std::fs;
use std::path::Path;

use crate::asar;

pub fn apply(resources: &Path, patcher: &Path) -> Result<(), String> {
    let app = resources.join("app.asar");
    let original = resources.join("_app.asar");

    // Déjà modifié (Strata, Vencord officiel…) : on garde le _app.asar d'origine
    if !original.exists() {
        if !app.is_file() {
            return Err(format!("app.asar introuvable dans {}", resources.display()));
        }
        fs::rename(&app, &original).map_err(|e| format!("Impossible de renommer app.asar : {e}"))?;
    }

    if let Err(e) = fs::write(&app, asar::patch_asar(patcher)) {
        // Rien ne doit rester à moitié fait : Discord retrouve son fichier d'origine
        let _ = fs::remove_file(&app);
        let _ = fs::rename(&original, &app);
        return Err(format!("Impossible d'écrire app.asar : {e}"));
    }
    Ok(())
}

pub fn remove(resources: &Path) -> Result<(), String> {
    let app = resources.join("app.asar");
    let original = resources.join("_app.asar");
    if !original.exists() {
        return Err("Ce Discord n'est pas modifié : il n'y a rien à retirer.".into());
    }
    if app.exists() {
        fs::remove_file(&app).map_err(|e| format!("Impossible de supprimer app.asar : {e}"))?;
    }
    fs::rename(&original, &app).map_err(|e| format!("Impossible de restaurer app.asar : {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discord::{find, latest_app_dir, State};
    use crate::paths::Paths;
    use std::path::PathBuf;

    const DISCORD_ASAR: &[u8] = b"le vrai app.asar de Discord";

    // Faux %LOCALAPPDATA% et %APPDATA% avec un Discord installé, deux versions
    fn sandbox() -> (tempfile::TempDir, Paths, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths { local_appdata: dir.path().join("Local"), appdata: dir.path().join("Roaming") };
        for version in ["app-1.0.9188", "app-1.0.9264"] {
            let resources = paths.local_appdata.join("Discord").join(version).join("resources");
            fs::create_dir_all(&resources).unwrap();
            fs::write(resources.join("app.asar"), DISCORD_ASAR).unwrap();
        }
        let resources = paths.local_appdata.join("Discord/app-1.0.9264/resources");
        (dir, paths, resources)
    }

    #[test]
    fn picks_the_newest_version() {
        let (_dir, paths, _) = sandbox();
        fs::create_dir_all(paths.local_appdata.join("Discord/app-1.0.10000/resources")).unwrap();
        let latest = latest_app_dir(&paths.local_appdata.join("Discord")).unwrap();
        assert!(latest.ends_with("app-1.0.10000"), "{latest:?}");
    }

    #[test]
    fn install_then_uninstall() {
        let (_dir, paths, resources) = sandbox();
        assert_eq!(find(&paths)[0].state, State::Original);

        apply(&resources, &paths.patcher()).unwrap();
        assert_eq!(fs::read(resources.join("_app.asar")).unwrap(), DISCORD_ASAR);
        let install = &find(&paths)[0];
        assert_eq!(install.state, State::Strata);
        assert_eq!(install.version, "1.0.9264");

        // Réinstaller (réparer) ne doit pas écraser le Discord d'origine sauvegardé
        apply(&resources, &paths.patcher()).unwrap();
        assert_eq!(fs::read(resources.join("_app.asar")).unwrap(), DISCORD_ASAR);

        remove(&resources).unwrap();
        assert_eq!(fs::read(resources.join("app.asar")).unwrap(), DISCORD_ASAR);
        assert!(!resources.join("_app.asar").exists());
        assert_eq!(find(&paths)[0].state, State::Original);
    }

    #[test]
    fn replaces_official_vencord() {
        let (_dir, paths, resources) = sandbox();
        let vencord = paths.appdata.join("Vencord/dist/patcher.js");
        apply(&resources, &vencord).unwrap();
        assert_eq!(find(&paths)[0].state, State::Other);

        apply(&resources, &paths.patcher()).unwrap();
        assert_eq!(find(&paths)[0].state, State::Strata);
        assert_eq!(fs::read(resources.join("_app.asar")).unwrap(), DISCORD_ASAR);
    }

    // Lecture seule sur la vraie machine : cargo test -- --ignored --nocapture
    #[test]
    #[ignore]
    fn detects_the_real_discord() {
        let paths = Paths::from_env().unwrap();
        for install in find(&paths) {
            println!("{install:#?}");
        }
    }

    #[test]
    fn nothing_to_remove_on_original_discord() {
        let (_dir, _paths, resources) = sandbox();
        assert!(remove(&resources).is_err());
        assert_eq!(fs::read(resources.join("app.asar")).unwrap(), DISCORD_ASAR);
    }
}
