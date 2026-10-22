<div align="center">
  <img src="src-tauri/icons/128x128@2x.png" alt="Icône Strata Launcher" width="120">

  <p align="center">
    <img src="https://img.shields.io/badge/Tauri-2.12-24C8DB?logo=tauri&amp;logoColor=white" alt="Tauri 2.12">
    <img src="https://img.shields.io/badge/Rust-2021-000000?logo=rust&amp;logoColor=white" alt="Rust 2021">
    <img src="https://img.shields.io/badge/Windows-10%20%2F%2011-0078D4?logo=windows&amp;logoColor=white" alt="Windows 10 / 11">
    <img src="https://img.shields.io/badge/HTML%20%2F%20CSS%20%2F%20JS-sans%20framework-E34F26?logo=html5&amp;logoColor=white" alt="HTML / CSS / JS sans framework">
  </p>

  <p align="center">
    <img src="https://img.shields.io/badge/Accent-%237C6CFF-7C6CFF" alt="Accent #7C6CFF">
    <img src="https://img.shields.io/badge/Fond-%230A0B0F-0A0B0F" alt="Fond #0A0B0F">
    <img src="https://img.shields.io/badge/Licence-GPL--3.0-A42E2B?logo=gnu&amp;logoColor=white" alt="Licence GPL-3.0">
  </p>

  <p align="center">
    <i>Installeur de Strata, une distribution de Vencord pour Discord.</i><br>
    Dépôt  <a href="https://github.com/DevRedious/strata-launcher">DevRedious/strata-launcher</a>
  </p>
</div>

---

> **État : en développement.** La logique est écrite et testée, l'interface est en place.
> Restent le test réel d'installation, la compilation dans GitHub Actions, la signature du
> programme et la passe visuelle. Pas encore de version à télécharger.

## Présentation

[Strata](https://github.com/DevRedious/strata) est Vencord plus un thème et un menu de réglages,
publié dans les releases de `DevRedious/strata`. Ce launcher l'installe sur Discord en un clic :

- il trouve les Discord installés (Stable, PTB, Canary) et indique pour chacun s'il est d'origine,
  déjà sous Strata, ou modifié par autre chose (Vencord officiel, par exemple) ;
- **Installer** télécharge la dernière release dans `%APPDATA%\Strata\dist`, ferme Discord, le
  branche sur Strata et le relance ;
- **Réparer** refait la même chose, **Désinstaller** rend Discord à son état d'origine.

Les mises à jour suivantes passent par l'outil de mise à jour intégré à Vencord, et Strata se
réapplique tout seul quand Discord se met à jour. On ne rouvre le launcher que pour réparer ou
désinstaller. Les réglages et plugins Vencord existants (`%APPDATA%\Vencord\settings`) sont
conservés.

## Comment Discord est modifié

Exactement comme le fait l'installeur officiel de Vencord : `resources/app.asar` est renommé en
`_app.asar`, et une petite archive `app.asar` de quelques centaines d'octets prend sa place. Elle
contient un `index.js` qui charge `%APPDATA%\Strata\dist\patcher.js`. Un test vérifie que
l'archive écrite est identique, octet pour octet, à celle de l'installeur officiel.

## Antivirus

Un programme qui ferme Discord et modifie ses fichiers ressemble, pour un antivirus, à un
logiciel malveillant. Le launcher fait donc le minimum, et de façon transparente :

- aucun droit administrateur : Discord est installé dans `%LOCALAPPDATA%` ;
- un seul exe, ni compressé ni obscurci ; rien ne reste en arrière-plan ;
- téléchargements par le TLS de Windows, compatible avec l'inspection HTTPS des antivirus ;
- code source public, binaires compilés par GitHub Actions ; signature prévue via
  [SignPath Foundation](https://signpath.org/).

## Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org). Rôles, binaires concernés et confidentialité : [CODE_SIGNING_POLICY.md](CODE_SIGNING_POLICY.md). La signature n'est pas encore active.

## Développement

```bash
pnpm install
STRATA_DEMO=1 pnpm dev        # vraie fenêtre, Discord simulés : aucun risque
pnpm dev                      # vraie fenêtre, vrais Discord : les boutons agissent
cd src-tauri && cargo test    # tests (faux Discord dans un dossier temporaire)
cargo test -- --ignored --nocapture   # lecture seule sur la machine et téléchargement réel
pnpm build                    # exe portable : src-tauri/target/release/strata-launcher.exe
```

`pnpm install` active les crochets Git (Husky, `.husky/`) : `cargo fmt --check` avant chaque commit, `cargo clippy` et `cargo test` avant chaque push. La CI refait ces contrôles.

| Chemin | Rôle |
|---|---|
| `src/` | Interface : HTML, CSS et JS, sans framework ni compilation. |
| `src-tauri/src/discord.rs` | Discord installés, état, fermeture et relance. |
| `src-tauri/src/patch.rs` | Brancher Discord sur Strata ou le rendre d'origine. |
| `src-tauri/src/asar.rs` | La petite archive `app.asar`. |
| `src-tauri/src/release.rs` | Dernière release de Strata et téléchargement. |
| `src-tauri/src/demo.rs` | Mode démo (`STRATA_DEMO=1`). |
| `preview/` | Aperçu dans un navigateur, avec des données simulées (`node preview/build.mjs`). |

## Avertissement

Comme tout client modifié, Strata est contraire aux conditions d'utilisation de Discord ; chacun
l'installe en connaissance de cause. Discord est une marque de Discord Inc. ; ce projet n'a aucun
lien avec Discord Inc. ni avec les auteurs de Vencord.

Licence [GPL-3.0-or-later](LICENSE).
