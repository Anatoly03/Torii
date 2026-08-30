/**
 * @brief Part of the automatic updates pipeline. This will read the environment variable
 * `TAURI_SIGNING_PUBLIC_KEY` and attach it to the `tauri.conf.json` file.
 * 
 * @see https://v2.tauri.app/plugin/updater/
 * 
 * @note This script is intended to be run in a GitHub Actions workflow, and not locally. It
 * will overwrite the `tauri.conf.json` file. This change is not meant to be committed.
 */

import * as fs from "fs";

/**
 * @brief We use the environment variable `TAURI_SIGNING_PUBLIC_KEY`
 */
const PUBLIC_KEY = process.env.TAURI_SIGNING_PUBLIC_KEY;

/**
 * @brief Path to the Tauri configuration file.
 */
const TAURI_CONFIG_PATH = "torii-desktop/tauri.conf.json";

/**
 * @brief Appends the information as specified by the Tauri Updater plugin to the Tauri
 * configuration file.
 * 
 * @see 
 */
function appendUpdaterConfig() {
    const tauriConfig = JSON.parse(fs.readFileSync(TAURI_CONFIG_PATH, "utf-8"));
    tauriConfig.bundle.createUpdaterArtifacts = true;
    tauriConfig.plugins.updater.pubkey = PUBLIC_KEY;
    fs.writeFileSync(TAURI_CONFIG_PATH, JSON.stringify(tauriConfig, null, 4));

    console.log('Public Key: ', PUBLIC_KEY);
}

// Run the change updater configuration.
appendUpdaterConfig();