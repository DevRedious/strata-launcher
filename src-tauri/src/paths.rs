// Emplacements utilisés par le launcher. Tout passe par cette structure pour que
// les tests travaillent dans un dossier temporaire, loin du vrai Discord.

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Paths {
    /// %LOCALAPPDATA% : Discord y installe ses versions (Discord, DiscordPTB…)
    pub local_appdata: PathBuf,
    /// %APPDATA% : Strata y range ses fichiers, Vencord ses réglages
    pub appdata: PathBuf,
}

impl Paths {
    pub fn from_env() -> Result<Self, String> {
        let var = |name: &str| {
            std::env::var_os(name).map(PathBuf::from).ok_or_else(|| {
                crate::i18n::tr(
                    &format!("Variable d'environnement {name} introuvable"),
                    &format!("Environment variable {name} not found"),
                )
            })
        };
        Ok(Self {
            local_appdata: var("LOCALAPPDATA")?,
            appdata: var("APPDATA")?,
        })
    }

    /// Fichiers de Strata, à part de ceux de Vencord : relancer l'installeur
    /// officiel de Vencord ne les écrase pas. L'outil de mise à jour les remplace sur place.
    pub fn dist(&self) -> PathBuf {
        self.appdata.join("Strata").join("dist")
    }

    pub fn patcher(&self) -> PathBuf {
        self.dist().join("patcher.js")
    }
}
