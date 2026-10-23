// Interface du launcher : liste les Discord installés et lance les opérations
// Rust (src-tauri/src/lib.rs) par invoke. Pas de bibliothèque ni de compilation.

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// Langues proposées, dans l'ordre de la liste. Pour en ajouter une : une ligne
// ici, ses textes dans TEXTS, et ses messages dans src-tauri/src/i18n.rs.
// Drapeaux en SVG : Windows n'affiche pas les émojis de drapeaux (il écrit « FR »)
const FLAGS = {
    fr: '<svg viewBox="0 0 3 2" preserveAspectRatio="xMidYMid slice"><path fill="#0055a4" d="M0 0h1v2H0z"/><path fill="#fff" d="M1 0h1v2H1z"/><path fill="#ef4135" d="M2 0h1v2H2z"/></svg>',
    gb: '<svg viewBox="0 0 60 30" preserveAspectRatio="xMidYMid slice"><clipPath id="gb-t"><path d="M30 15h30v15zv15H0zH0V0zV0h30z"/></clipPath><path fill="#012169" d="M0 0h60v30H0z"/><path stroke="#fff" stroke-width="6" d="m0 0 60 30m0-30L0 30"/><path stroke="#c8102e" stroke-width="4" d="m0 0 60 30m0-30L0 30" clip-path="url(#gb-t)"/><path stroke="#fff" stroke-width="10" d="M30 0v30M0 15h60"/><path stroke="#c8102e" stroke-width="6" d="M30 0v30M0 15h60"/></svg>',
};

const LANGUAGES = [
    { code: "fr", name: "Français", flag: FLAGS.fr },
    { code: "en", name: "English", flag: FLAGS.gb },
];

// Textes de l'interface. Les messages des opérations viennent du Rust, dans la
// langue transmise par set_language.
const TEXTS = {
    fr: {
        subtitle: "Installe Strata sur Discord : thème, réglages et mises à jour automatiques.",
        searching: "Recherche de Discord…",
        empty: "Aucun Discord trouvé. Installe Discord depuis discord.com, puis relance cette application.",
        notice: "Discord va se fermer puis se relancer : un appel en cours sera coupé. Tes réglages et tes plugins Vencord sont conservés.",
        tos: "Comme tout client modifié, Strata est contraire aux conditions d'utilisation de Discord.",
        running: "Ouvert",
        uninstall: "Désinstaller",
        preparing: "Préparation…",
        latest: "Dernière version publiée",
        version: hash => `Version ${hash}`,
        offline: "Hors ligne",
        continue: "Continuer",
        language: "Langue",
        states: {
            original: { badge: "Discord d'origine", action: "Installer Strata" },
            strata: { badge: "Strata installé", action: "Réparer" },
            other: { badge: "Vencord ou autre mod", action: "Remplacer par Strata" },
        },
    },
    en: {
        subtitle: "Installs Strata on Discord: theme, settings and automatic updates.",
        searching: "Looking for Discord…",
        empty: "No Discord found. Install Discord from discord.com, then start this app again.",
        notice: "Discord will close and restart: an ongoing call will be cut. Your Vencord settings and plugins are kept.",
        tos: "Like any modified client, Strata goes against Discord's Terms of Service.",
        running: "Running",
        uninstall: "Uninstall",
        preparing: "Preparing…",
        latest: "Latest published version",
        version: hash => `Version ${hash}`,
        offline: "Offline",
        continue: "Continue",
        language: "Language",
        states: {
            original: { badge: "Original Discord", action: "Install Strata" },
            strata: { badge: "Strata installed", action: "Repair" },
            other: { badge: "Vencord or another mod", action: "Replace with Strata" },
        },
    },
};

const LANG_KEY = "strata-launcher-lang";
let lang = localStorage.getItem(LANG_KEY);
let t = TEXTS[lang] ?? TEXTS.fr;

const $ = id => document.getElementById(id);
let busy = false;
let lastInstalls = null;
let release = null;

// Liste déroulante : un bouton qui affiche le choix, une liste qui s'ouvre dessous.
// Clavier : Entrée ou Espace ouvre, flèches pour se déplacer, Échap ferme.
function dropdown(root, { label, value, onChange }) {
    root.replaceChildren();
    const button = document.createElement("button");
    button.type = "button";
    button.className = "dropdown-button";
    button.setAttribute("aria-haspopup", "listbox");
    const list = document.createElement("ul");
    list.className = "dropdown-list";
    list.setAttribute("role", "listbox");
    list.hidden = true;
    root.append(button, list);

    let current = value;
    const render = () => {
        const language = LANGUAGES.find(l => l.code === current);
        button.innerHTML = `<span class="dropdown-flag" aria-hidden="true">${language.flag}</span><span class="dropdown-name"></span><span class="dropdown-caret" aria-hidden="true"></span>`;
        button.querySelector(".dropdown-name").textContent = language.name;
        button.setAttribute("aria-label", `${label()} : ${language.name}`);
        list.replaceChildren(...LANGUAGES.map(l => {
            const item = document.createElement("li");
            item.setAttribute("role", "option");
            item.setAttribute("aria-selected", String(l.code === current));
            item.lang = l.code;
            item.tabIndex = -1;
            item.innerHTML = `<span class="dropdown-flag" aria-hidden="true">${l.flag}</span><span></span>`;
            item.lastChild.textContent = l.name;
            item.addEventListener("click", () => choose(l.code));
            return item;
        }));
    };
    const close = () => {
        list.hidden = true;
        button.setAttribute("aria-expanded", "false");
        root.classList.remove("open");
    };
    const open = () => {
        list.hidden = false;
        button.setAttribute("aria-expanded", "true");
        root.classList.add("open");
        list.querySelector('[aria-selected="true"]')?.focus();
    };
    const choose = code => {
        close();
        button.focus();
        if (code !== current) {
            current = code;
            render();
            onChange(code);
        }
    };

    button.addEventListener("click", () => (list.hidden ? open() : close()));
    button.addEventListener("keydown", e => {
        if (e.key === "ArrowDown" || e.key === "ArrowUp") {
            e.preventDefault();
            open();
        }
    });
    list.addEventListener("keydown", e => {
        const items = [...list.children];
        const index = items.indexOf(document.activeElement);
        if (e.key === "ArrowDown") items[Math.min(index + 1, items.length - 1)].focus();
        else if (e.key === "ArrowUp") items[Math.max(index - 1, 0)].focus();
        else if (e.key === "Enter" || e.key === " ") choose(LANGUAGES[index].code);
        else if (e.key === "Escape") { close(); button.focus(); }
        else return;
        e.preventDefault();
    });
    document.addEventListener("click", e => { if (!root.contains(e.target)) close(); });

    render();
    return { refresh: render, set: code => { current = code; render(); } };
}

function setStatus(text, kind = "progress") {
    const status = $("status");
    status.hidden = !text;
    status.dataset.kind = kind;
    $("status-text").textContent = text;
}

function render(installs) {
    lastInstalls = installs;
    const list = $("installs");
    list.replaceChildren();

    if (!installs.length) {
        const empty = document.createElement("p");
        empty.className = "empty";
        empty.textContent = t.empty;
        list.append(empty);
        return;
    }

    for (const install of installs) {
        const card = $("install-card").content.firstElementChild.cloneNode(true);
        const state = t.states[install.state];
        card.dataset.state = install.state;
        card.querySelector(".label").textContent = install.label;
        card.querySelector(".version").textContent = `v${install.version}`;
        // Vraie icône de ce Discord (lue dans son installation), sinon l'icône neutre
        if (install.icon) {
            card.querySelector(".card-icon img").src = install.icon;
            card.querySelector(".card-icon").classList.add("real");
        }
        card.querySelector(".badge").textContent = state.badge;
        card.querySelector(".running").textContent = t.running;
        card.querySelector(".running").hidden = !install.running;

        const installButton = card.querySelector(".install");
        installButton.textContent = state.action;
        installButton.addEventListener("click", () => run("install", install));

        const uninstallButton = card.querySelector(".uninstall");
        uninstallButton.textContent = t.uninstall;
        uninstallButton.hidden = install.state !== "strata";
        uninstallButton.addEventListener("click", () => run("uninstall", install));

        list.append(card);
    }
}

function renderRelease() {
    $("release").title = t.latest;
    $("release").textContent = release === null ? "…" : release ? t.version(release.hash) : t.offline;
}

// Textes fixes (data-i18n), puis tout ce qui est construit par le script
function applyTexts() {
    document.documentElement.lang = lang;
    for (const el of document.querySelectorAll("[data-i18n]")) {
        if (typeof t[el.dataset.i18n] === "string") el.textContent = t[el.dataset.i18n];
    }
    headerPicker?.set(lang);
    renderRelease();
    if (lastInstalls) render(lastInstalls);
}

async function setLanguage(next) {
    lang = next;
    t = TEXTS[lang];
    localStorage.setItem(LANG_KEY, lang);
    await invoke("set_language", { lang });
    // Le dernier résultat affiché est dans l'ancienne langue : on l'efface
    if (!busy) setStatus("");
    applyTexts();
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
        setStatus(t.preparing);
        setStatus(await invoke(command, { branch: install.branch }), "success");
    } catch (error) {
        setStatus(String(error), "error");
    } finally {
        busy = false;
        document.body.classList.remove("busy");
        refresh();
    }
}

function start() {
    document.body.classList.remove("choosing");
    $("language").hidden = true;
    applyTexts();
    refresh();
    invoke("latest_release")
        .then(r => { release = r; })
        .catch(() => { release = false; })
        .finally(renderRelease);
}

listen("progress", event => setStatus(event.payload));

// Langue proposée au premier lancement : celle de Windows si elle est disponible
const systemLanguage = () => {
    const code = navigator.language?.toLowerCase().split("-")[0];
    return TEXTS[code] ? code : "en";
};

let headerPicker = null;

function createHeaderPicker() {
    headerPicker = dropdown($("lang-switch"), { label: () => t.language, value: lang, onChange: setLanguage });
}

if (TEXTS[lang]) {
    invoke("set_language", { lang });
    createHeaderPicker();
    start();
} else {
    // Premier lancement : la langue d'abord, dans une liste déroulante
    lang = systemLanguage();
    t = TEXTS[lang];
    document.body.classList.add("choosing");
    $("language").hidden = false;
    const showContinue = () => { $("language-continue").textContent = t.continue; };
    dropdown($("language-picker"), {
        label: () => t.language,
        value: lang,
        onChange: code => { lang = code; t = TEXTS[code]; showContinue(); },
    });
    showContinue();
    $("language-continue").addEventListener("click", async () => {
        await setLanguage(lang);
        createHeaderPicker();
        start();
    });
    document.querySelector("#language-picker .dropdown-button").focus();
}
