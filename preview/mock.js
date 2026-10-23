// Données simulées pour l'aperçu dans un navigateur (pas embarqué dans l'exe).
// #langue : premier lancement (efface la langue gardée), #en : interface anglaise.
const scene = location.hash.slice(1) || "liste";
// #langue-ouverte : première étape, liste déroulante ouverte ; #liste-ouverte : celle de l'en-tête
if (scene.startsWith("langue")) localStorage.removeItem("strata-launcher-lang");
if (scene.endsWith("-ouverte")) addEventListener("load", () => setTimeout(() => document.querySelector(".dropdown:not([hidden] *) .dropdown-button:not(:empty)")?.click(), 100));
if (scene === "en") localStorage.setItem("strata-launcher-lang", "en");
else if (!scene.startsWith("langue")) localStorage.setItem("strata-launcher-lang", "fr");
const installs = [
    { branch: "stable", label: "Discord", version: "1.0.9264", state: "original", running: true },
    { branch: "ptb", label: "Discord PTB", version: "1.0.1182", state: "strata", running: false },
];
window.__TAURI__ = {
    core: {
        invoke: async command => {
            if (command === "installs") return scene === "vide" ? [] : installs;
            if (command === "latest_release") return { name: "Strata 70860a5", hash: "70860a5" };
            if (command === "set_language") return;
            return new Promise(() => {});
        },
    },
    event: { listen: async () => {} },
};
if (scene === "installation") {
    addEventListener("load", () => setTimeout(() => {
        document.body.classList.add("busy");
        document.getElementById("status").hidden = false;
        document.getElementById("status").dataset.kind = "progress";
        document.getElementById("status-text").textContent = "Téléchargement de Strata…";
    }, 50));
}
