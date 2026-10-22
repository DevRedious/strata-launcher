// Interface du launcher : liste les Discord installés et lance les opérations
// Rust (src-tauri/src/lib.rs) par invoke. Pas de bibliothèque ni de compilation.

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const STATES = {
    original: { badge: "Discord d'origine", action: "Installer Strata" },
    strata: { badge: "Strata installé", action: "Réparer" },
    other: { badge: "Vencord ou autre mod", action: "Remplacer par Strata" },
};

const $ = id => document.getElementById(id);
let busy = false;

function setStatus(text, kind = "progress") {
    const status = $("status");
    status.hidden = !text;
    status.dataset.kind = kind;
    $("status-text").textContent = text;
}

function render(installs) {
    const list = $("installs");
    list.replaceChildren();

    if (!installs.length) {
        const empty = document.createElement("p");
        empty.className = "empty";
        empty.textContent = "Aucun Discord trouvé. Installe Discord depuis discord.com, puis relance cette application.";
        list.append(empty);
        return;
    }

    for (const install of installs) {
        const card = $("install-card").content.firstElementChild.cloneNode(true);
        const state = STATES[install.state];
        card.dataset.state = install.state;
        card.querySelector(".label").textContent = install.label;
        card.querySelector(".version").textContent = install.version;
        card.querySelector(".badge").textContent = state.badge;
        card.querySelector(".running").hidden = !install.running;

        const installButton = card.querySelector(".install");
        installButton.textContent = state.action;
        installButton.addEventListener("click", () => run("install", install));

        const uninstallButton = card.querySelector(".uninstall");
        uninstallButton.hidden = install.state !== "strata";
        uninstallButton.addEventListener("click", () => run("uninstall", install));

        list.append(card);
    }
}

async function refresh() {
    try {
        render(await invoke("installs"));
    } catch (error) {
        setStatus(String(error), "error");
    }
}

async function run(command, install) {
    if (busy) return;
    busy = true;
    document.body.classList.add("busy");
    try {
        setStatus("Préparation…");
        setStatus(await invoke(command, { branch: install.branch }), "success");
    } catch (error) {
        setStatus(String(error), "error");
    } finally {
        busy = false;
        document.body.classList.remove("busy");
        refresh();
    }
}

listen("progress", event => setStatus(event.payload));

invoke("latest_release")
    .then(release => { $("release").textContent = `Version ${release.hash}`; })
    .catch(() => { $("release").textContent = "Hors ligne"; });

refresh();
