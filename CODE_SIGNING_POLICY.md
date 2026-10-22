# Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org).

*La signature n'est pas encore en place : la demande à SignPath Foundation est en préparation. Tant que ce paragraphe est là, les exe publiés ne sont pas signés.*

## Ce qui est signé

Uniquement `strata-launcher.exe`, compilé par GitHub Actions ([`.github/workflows/build.yml`](.github/workflows/build.yml)) à partir du code de ce dépôt, pour chaque release `v*`. Aucun binaire compilé sur un poste de développement n'est signé ni publié.

Le launcher télécharge ensuite les fichiers de [Strata](https://github.com/DevRedious/strata) (JavaScript et CSS chargés par Discord) depuis les releases de `DevRedious/strata`. Ces fichiers ne sont pas des exécutables Windows et ne sont pas couverts par cette signature.

## Rôles

| Rôle | Membres |
|---|---|
| Auteur, relecteur et approbateur des signatures | [DevRedious](https://github.com/DevRedious) |

Toute contribution extérieure (pull request) est relue avant d'être fusionnée. Chaque demande de signature est approuvée à la main. L'authentification à deux facteurs est activée sur GitHub et sur SignPath.

## Ce que fait le programme

Le launcher installe, répare ou désinstalle Strata sur les clients Discord de l'utilisateur, à sa demande (un bouton par opération) :

- il ferme le client Discord concerné, après avoir prévenu qu'un appel en cours sera coupé ;
- il renomme `resources\app.asar` en `_app.asar` dans le dossier de Discord et écrit à sa place une petite archive qui charge Strata, comme l'installeur officiel de Vencord ;
- « Désinstaller » rend à Discord son fichier d'origine et supprime les fichiers de Strata ;
- il ne demande aucun droit administrateur, ne modifie rien en dehors du dossier de Discord et de `%APPDATA%\Strata`, et ne laisse aucun programme en arrière-plan.

## Confidentialité

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

Précisément : au démarrage, le launcher demande à `api.github.com` le nom de la dernière release de Strata pour l'afficher ; quand l'utilisateur installe ou répare, il télécharge les fichiers de cette release depuis `github.com`. Ces requêtes ne contiennent aucune donnée personnelle (seulement l'en-tête `User-Agent: StrataLauncher/<version>`). La [déclaration de confidentialité de GitHub](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement) s'applique à ces échanges.
