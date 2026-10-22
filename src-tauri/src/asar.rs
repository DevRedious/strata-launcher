// Petite archive app.asar qui fait charger Strata à Discord : un index.js qui
// appelle le patcher.js de Strata, et un package.json. Même contenu, octet pour
// octet, que celle de l'installeur officiel de Vencord (app_asar.go).
//
// Format asar : un en-tête « pickle » (entiers 32 bits little-endian), l'index
// des fichiers en JSON aligné sur 4 octets, puis le contenu des fichiers.

use std::path::Path;

const PACKAGE_JSON: &str = "{\n\t\"name\": \"discord\",\n\t\"main\": \"index.js\"\n}";

pub fn patch_asar(patcher: &Path) -> Vec<u8> {
    // Chemin échappé comme une chaîne JSON : R:\Code → "R:\\Code"
    let quoted = serde_json::to_string(&patcher.to_string_lossy()).expect("chaîne sérialisable");
    let index_js = format!("require({quoted})");

    let header = format!(
        r#"{{"files":{{"index.js":{{"size":{index},"offset":"0"}},"package.json":{{"size":{package},"offset":"{index}"}}}}}}"#,
        index = index_js.len(),
        package = PACKAGE_JSON.len(),
    );
    let padded = (header.len() + 3) & !3;

    let mut out = Vec::with_capacity(16 + padded + index_js.len() + PACKAGE_JSON.len());
    out.extend_from_slice(&4u32.to_le_bytes());
    out.extend_from_slice(&((padded + 8) as u32).to_le_bytes());
    out.extend_from_slice(&((padded + 4) as u32).to_le_bytes());
    out.extend_from_slice(&(header.len() as u32).to_le_bytes());
    out.extend_from_slice(header.as_bytes());
    out.resize(16 + padded, 0);
    out.extend_from_slice(index_js.as_bytes());
    out.extend_from_slice(PACKAGE_JSON.as_bytes());
    out
}

// Chemin du patcher.js qu'une archive fait charger, s'il s'agit d'une archive
// de ce type (Strata, Vencord officiel ou installation de développement)
pub fn patcher_of(asar: &[u8]) -> Option<String> {
    // Le vrai app.asar de Discord pèse plusieurs Mo et contient ses propres require()
    if asar.len() > 4096 {
        return None;
    }
    let text = String::from_utf8_lossy(asar);
    let start = text.find("require(")? + "require(".len();
    let end = start + text[start..].find(')')?;
    serde_json::from_str(&text[start..end]).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Archive écrite par VencordInstallerCli pour R:\Code\strata\dist\patcher.js
    const REFERENCE: &[u8] = include_bytes!("../tests/fixtures/vencord-installer-app.asar");

    #[test]
    fn identical_to_the_official_installer() {
        assert_eq!(patch_asar(Path::new(r"R:\Code\strata\dist\patcher.js")), REFERENCE);
    }

    #[test]
    fn reads_back_the_patcher_path() {
        assert_eq!(
            patcher_of(REFERENCE).as_deref(),
            Some(r"R:\Code\strata\dist\patcher.js")
        );
        let accents = Path::new(r"C:\Users\Élodie\AppData\Roaming\Strata\dist\patcher.js");
        assert_eq!(
            patcher_of(&patch_asar(accents)).as_deref(),
            Some(accents.to_str().unwrap())
        );
    }

    #[test]
    fn ignores_discords_own_archive() {
        assert_eq!(patcher_of(b"\x04\0\0\0 not a patch archive"), None);
    }
}
