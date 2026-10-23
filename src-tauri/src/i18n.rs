// Langue des messages renvoyés à l'interface (étapes, résultats, erreurs). C'est
// l'interface qui la choisit (sélecteur au premier lancement) et la transmet par
// la commande set_language ; le français reste la langue par défaut.

use std::sync::atomic::{AtomicBool, Ordering};

static ENGLISH: AtomicBool = AtomicBool::new(false);

pub fn set(lang: &str) {
    ENGLISH.store(lang == "en", Ordering::Relaxed);
}

/// Le message dans la langue choisie : `tr("Installation…", "Installing…")`
pub fn tr(fr: &str, en: &str) -> String {
    if ENGLISH.load(Ordering::Relaxed) { en } else { fr }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_chosen_language() {
        set("en");
        assert_eq!(tr("Installation…", "Installing…"), "Installing…");
        set("fr");
        assert_eq!(tr("Installation…", "Installing…"), "Installation…");
    }
}
