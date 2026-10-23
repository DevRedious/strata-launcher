// Dernière release de Strata sur GitHub : la même que lit l'outil de mise à jour
// de Vencord (src/main/updater/http.ts dans DevRedious/strata).

use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};

use crate::i18n::tr;

pub const REPO: &str = "DevRedious/strata";
const USER_AGENT: &str = concat!(
    "StrataLauncher/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/DevRedious/strata-launcher)"
);
// Fichiers chargés par Discord (et leurs .map) : mêmes préfixes que VENCORD_FILES
const FILES: [&str; 4] = ["patcher.js", "preload.js", "renderer.js", "renderer.css"];
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    name: String,
    assets: Vec<Asset>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Release {
    pub name: String,
    /// Dernier mot du nom, comme le lit l'outil de mise à jour
    pub hash: String,
}

// TLS de Windows et ses certificats racine : un antivirus qui inspecte le HTTPS
// (Norton…) ou un proxy d'entreprise y ajoute le sien, le téléchargement passe
fn agent() -> &'static ureq::Agent {
    static AGENT: OnceLock<ureq::Agent> = OnceLock::new();
    AGENT.get_or_init(|| {
        let tls = TlsConfig::builder()
            .provider(TlsProvider::NativeTls)
            .root_certs(RootCerts::PlatformVerifier)
            .build();
        ureq::Agent::config_builder().tls_config(tls).build().into()
    })
}

fn get(url: &str) -> Result<ureq::http::Response<ureq::Body>, String> {
    agent()
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| {
            tr(
                &format!("Échec du téléchargement ({url}) : {e}"),
                &format!("Download failed ({url}): {e}"),
            )
        })
}

fn fetch() -> Result<GithubRelease, String> {
    get(&format!("https://api.github.com/repos/{REPO}/releases/latest"))?
        .body_mut()
        .read_json()
        .map_err(|e| {
            tr(
                &format!("Réponse de GitHub illisible : {e}"),
                &format!("Unreadable response from GitHub: {e}"),
            )
        })
}

fn hash_of(name: &str) -> String {
    name.rsplit(' ').next().unwrap_or(name).to_string()
}

pub fn latest() -> Result<Release, String> {
    let release = fetch()?;
    Ok(Release {
        hash: hash_of(&release.name),
        name: release.name,
    })
}

/// Télécharge les fichiers de la dernière release dans `dist`. Chaque fichier est
/// d'abord écrit à côté puis renommé : une coupure ne laisse pas de fichier tronqué.
pub fn download(dist: &Path) -> Result<Release, String> {
    let release = fetch()?;
    let assets: Vec<&Asset> = release
        .assets
        .iter()
        .filter(|a| FILES.iter().any(|f| a.name.starts_with(f)))
        .collect();
    if !FILES.iter().all(|f| assets.iter().any(|a| &a.name == f)) {
        return Err(tr(
            &format!("La version « {} » est incomplète.", release.name),
            &format!("Release “{}” is incomplete.", release.name),
        ));
    }

    fs::create_dir_all(dist).map_err(|e| {
        tr(
            &format!("Impossible de créer {} : {e}", dist.display()),
            &format!("Could not create {}: {e}", dist.display()),
        )
    })?;
    // Sans package.json, Node remonterait l'arborescence à la recherche d'un autre
    // (comme le fait l'installeur officiel de Vencord)
    fs::write(dist.join("package.json"), "{}").map_err(|e| e.to_string())?;

    for asset in assets {
        let bytes = get(&asset.browser_download_url)?
            .body_mut()
            .with_config()
            .limit(MAX_FILE_SIZE)
            .read_to_vec()
            .map_err(|e| {
                tr(
                    &format!("Téléchargement de {} interrompu : {e}", asset.name),
                    &format!("Download of {} interrupted: {e}", asset.name),
                )
            })?;
        let target = dist.join(&asset.name);
        let partial = dist.join(format!("{}.partial", asset.name));
        fs::write(&partial, bytes).map_err(|e| {
            tr(
                &format!("Écriture de {} impossible : {e}", asset.name),
                &format!("Could not write {}: {e}", asset.name),
            )
        })?;
        fs::rename(&partial, &target).map_err(|e| {
            tr(
                &format!("Écriture de {} impossible : {e}", asset.name),
                &format!("Could not write {}: {e}", asset.name),
            )
        })?;
    }

    Ok(Release {
        hash: hash_of(&release.name),
        name: release.name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_the_last_word() {
        assert_eq!(hash_of("Strata 70860a5"), "70860a5");
    }

    // Réseau réel, dans un dossier temporaire : cargo test -- --ignored
    #[test]
    #[ignore]
    fn downloads_the_real_release() {
        let dir = tempfile::tempdir().unwrap();
        let release = download(dir.path()).unwrap();
        for file in FILES {
            assert!(dir.path().join(file).metadata().unwrap().len() > 0, "{file} vide");
        }
        let banner = fs::read_to_string(dir.path().join("renderer.js")).unwrap();
        assert!(
            banner.starts_with(&format!("// Vencord {}\n", release.hash)),
            "bannière : {}",
            &banner[..40]
        );
        println!(
            "{} téléchargée : {:?}",
            release.name,
            fs::read_dir(dir.path())
                .unwrap()
                .map(|e| e.unwrap().file_name())
                .collect::<Vec<_>>()
        );
    }
}
