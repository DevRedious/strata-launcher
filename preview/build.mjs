// Génère preview/index.html à partir de src/index.html : mêmes fichiers, plus
// mock.js qui simule les commandes Rust. Ouvrir ensuite preview/index.html#liste,
// #installation ou #vide dans un navigateur. Pour essayer la vraie fenêtre sans
// risque, préférer le mode démo : STRATA_DEMO=1 pnpm dev.
import { readFileSync, writeFileSync } from "node:fs";

const root = new URL("..", import.meta.url);
const html = readFileSync(new URL("src/index.html", root), "utf8")
    .replace(/(href|src)="(?!\.\.\/)([\w.-]+\.(?:css|js|svg))"/g, '$1="../src/$2"')
    .replace('<script src="../src/main.js" defer></script>', '<script src="mock.js"></script>\n    <script src="../src/main.js" defer></script>');
writeFileSync(new URL("preview/index.html", root), html);
console.log("preview/index.html généré");
