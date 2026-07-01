/**
 * @brief Prints the Tauri version.
 */

import * as fs from "fs";

/**
 * @brief Path to the Tauri configuration file.
 */
const TAURI_CONFIG_PATH = "torii-desktop/tauri.conf.json";

/**
 * @brief Prints the Tauri version.
 */
function printVersion() {
    const tauriConfig = JSON.parse(fs.readFileSync(TAURI_CONFIG_PATH, "utf-8"));
    console.log(tauriConfig.version);
}

// Run the versioning fix.
printVersion();

