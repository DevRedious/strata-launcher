# Demande de signature à SignPath Foundation

Suivi de la demande de signature gratuite du launcher par [SignPath Foundation](https://signpath.org) : chaque exigence de leurs [conditions](https://signpath.org/terms), où on y répond, et les réponses du formulaire de [candidature](https://signpath.org/apply).

**État : pas encore envoyée.** On complète d'abord les points marqués « à faire ».

## Pourquoi SignPath

Windows (SmartScreen) et les antivirus (Norton…) se fient à la réputation d'un exe, et d'abord à sa signature. Une signature ne compte que si le certificat vient d'une autorité reconnue. SignPath Foundation signe gratuitement les projets open source avec un tel certificat, à son nom. Des outils proches de Strata sont déjà signés par eux : Spicetify (client Spotify), Millennium (client Steam), Pengu Loader (client League of Legends), Bloxstrap (Roblox). Estimation d'acceptation : environ 75 à 80 %, faute de statistiques publiées. Plan B en cas de refus : certificat Certum (à partir d'environ 49 €) ou pas de signature.

## Exigences

### Projet

| Exigence | Réponse | État |
|---|---|---|
| Ni logiciel malveillant ni programme indésirable | Code public ; Windows Defender ne signale rien sur l'exe de la CI ; le launcher prévient avant de fermer Discord et sait tout défaire | Fait |
| Licence OSI, sans double licence commerciale | GPL-3.0-or-later ([LICENSE](../LICENSE)) | Fait |
| Aucun composant propriétaire (bibliothèques système admises) | Dépendances open source (Tauri, ureq, sysinfo, serde) ; WebView2 est un composant de Windows | Fait |
| Activement maintenu | Commits réguliers, CI | Fait |
| Déjà publié sous la forme à signer | [Release v0.1.0](https://github.com/DevRedious/strata-launcher/releases/tag/v0.1.0) | Fait |
| Fonctionnement décrit sur la page de téléchargement | README, notes de release, [page de téléchargement du site](https://devredious.github.io/strata/en/download.html) | Fait |
| Le nom trouvable par une recherche Google | [devredious.github.io/strata](https://devredious.github.io/strata/), déclaré comme page d'accueil des deux dépôts ; l'indexation par Google prend quelques jours | Fait (indexation en cours) |

### Politique de signature (« Code signing policy »)

| Exigence | Réponse | État |
|---|---|---|
| L'intitulé exact « Code signing policy » sur la page d'accueil et les pages de téléchargement | README, [CODE_SIGNING_POLICY.md](../CODE_SIGNING_POLICY.md), notes de release (workflow), site : accueil, téléchargement et [page dédiée](https://devredious.github.io/strata/en/code-signing.html) | Fait |
| Mention « Free code signing provided by SignPath.io, certificate by SignPath Foundation » | Mêmes endroits | Fait |
| Rôles de l'équipe (auteurs, relecteurs, approbateurs) | CODE_SIGNING_POLICY.md | Fait |
| Déclaration de confidentialité | CODE_SIGNING_POLICY.md et [politique de confidentialité du site](https://devredious.github.io/strata/en/privacy.html) (modèle SignPath, détail de chaque connexion vérifiée dans le code) | Fait |

### Compilation et signature

| Exigence | Réponse | État |
|---|---|---|
| Binaire compilé automatiquement et de façon vérifiable depuis le dépôt | `.github/workflows/build.yml` | Fait |
| Tous les jobs sur des machines hébergées par GitHub | `windows-latest`, `ubuntu-latest` | Fait |
| L'artefact stocké comme artefact de workflow avant signature | `actions/upload-artifact` | Fait |
| Nom du produit identique au nom du projet, version cohérente | Métadonnées de l'exe : « Strata Launcher », version de `tauri.conf.json` ; le tag doit correspondre (vérifié par le workflow) | Fait |
| Approbation manuelle de chaque signature | Sur SignPath, après l'acceptation | Après acceptation |
| Relecture de toute contribution extérieure | Indiqué dans la politique | Fait |
| Authentification à deux facteurs, GitHub et SignPath, pour tous les membres | GitHub : activée (confirmé par DevRedious le 2026-10-22) ; SignPath : à l'activation du compte | Fait pour GitHub |

### Comportement du programme

| Exigence | Réponse | État |
|---|---|---|
| Aucune fonction d'attaque ou d'exploitation | Aucune | Fait |
| Rien qui compromette la confidentialité ou la sécurité | Pas de collecte ; requêtes vers GitHub seulement, sans donnée personnelle | Fait |
| Pas de modification du système sans avertissement | Avertissement affiché (appel coupé, Discord relancé) ; ne touche que le dossier de Discord et `%APPDATA%\Strata` | Fait |
| Désinstallation fournie | Bouton « Désinstaller » : rend à Discord son fichier d'origine, supprime les fichiers de Strata | Fait |

## Réponses du formulaire

Champs relevés sur le formulaire le 2026-10-22 (formulaire HubSpot). Les choix des menus sont recopiés tels quels.

| Champ | Réponse |
|---|---|
| Project Name | `Strata Launcher` |
| Repository URL | `https://github.com/DevRedious/strata-launcher` |
| Homepage URL | `https://devredious.github.io/strata/en/` |
| Download URL | `https://devredious.github.io/strata/en/download.html` (mentionne SignPath) |
| Privacy Policy URL | `https://devredious.github.io/strata/en/privacy.html` |
| Wikipedia URL | vide |
| Category | `Other` (choix : Accessibility, AI Tools, Audio, Developer Tools, Educational, Gaming, Graphics, Productivity, Science, Other) |
| Maintainer Type | `Individual maintainer(s)` |
| Build System | `GitHub Actions` |
| Primary Discovery Channel | `AI / LLM tools`, précision : `Recommended by Claude (AI assistant)` |
| First Name, Last Name, Email | ceux de DevRedious |
| Company Name | vide |
| Cases | Code of Conduct et stockage des données : cochées (obligatoires) ; communications commerciales : au choix |

**Tagline**

> A small open-source Windows installer that adds a theme and a settings menu to Discord through Strata, a Vencord distribution.

**Description**

> Strata Launcher installs, repairs and uninstalls Strata, an open-source (GPL-3.0) distribution of the Vencord Discord client mod that adds a dark theme and an in-app settings menu. On the user's explicit request it closes Discord after a warning, then loads Strata the same way the official Vencord installer does; uninstalling restores Discord's original file. It is a portable executable that needs no administrator rights, only contacts GitHub to download Strata, and collects no data. Every binary is built from source by GitHub Actions. Similar client-customization tools are already signed by SignPath Foundation, such as Spicetify, Millennium and Pengu Loader.

**Reputation**

> Strata Launcher is a new project (first release in October 2026) used by a small group of colleagues. It builds on Vencord (github.com/Vendicated/Vencord, about 14,300 stars and 2,700 forks), one of the most widely used Discord client mods, and reproduces its official installer's method byte for byte (covered by a test in the repository). Source, CI builds and releases are public: github.com/DevRedious/strata-launcher and github.com/DevRedious/strata. Project website: devredious.github.io/strata.

## Après l'acceptation

1. Activer l'authentification à deux facteurs sur le compte SignPath.
2. Ajouter le jeton d'API SignPath dans les secrets du dépôt (`SIGNPATH_API_TOKEN`).
3. Brancher `signpath/github-action-submit-signing-request` dans `build.yml` ([documentation](https://docs.signpath.io/trusted-build-systems/github)) avec l'identifiant d'organisation, le projet, la politique de signature et la configuration de l'artefact (un ZIP à la racine, comme les artefacts de GitHub).
4. Retirer de la politique et du site la mention « la signature n'est pas encore active ».
5. À chaque release : approuver la signature sur SignPath.
