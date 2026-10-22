// Données simulées pour l'aperçu dans un navigateur (pas embarqué dans l'exe)
const scene = location.hash.slice(1) || "liste";
const installs = [
    { branch: "stable", label: "Discord", version: "1.0.9264", state: "original", running: true },
    { branch: "ptb", label: "Discord PTB", version: "1.0.1182", state: "strata", running: false },
];
window.__TAURI__ = {
    core: {
        invoke: async command => {
            if (command === "installs") return scene === "vide" ? [] : installs;
            if (command === "latest_release") return { name: "Strata 70860a5", hash: "70860a5" };
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
