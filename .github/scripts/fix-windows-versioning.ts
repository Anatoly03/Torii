/**
 * @brief In previous publish builds, the versioning for pre-releases contained a string. We
 * intend to keep this string. On Windows however, the versioning is expected to be a number
 * @see https://github.com/Anatoly03/Torii/actions/runs/27463818799/job/81182445945
 * 
 * @note This script is intended to be run in a GitHub Actions workflow, and not locally. It
 * will overwrite the `tauri.conf.json` file with the "windows fixed" versioning. It is not
 * meant to be committed.
 * 
 * If you are a developer on Windows, in the future we plan to move this logic to `build.rs`
 * with a `cfg!(target_os = "windows")` check. Perhaps you should do it.
 */

import * as fs from "fs";

/**
 * @brief Path to the Tauri configuration file.
 */
const TAURI_CONFIG_PATH = "torii-desktop/tauri.conf.json";

/**
 * @param version Version provided by the repository in `tauri.conf.json`.
 * @return Version string with non-leading letters removed.
 * @example
 *
 * assert.equal(removeLetters("0.0.0-dev2"), "0.0.0-2");
 * assert.equal(removeLetters("0.0.1"), "0.0.1");
 * assert.equal(removeLetters("0.1.0-alpha"), "0.1.0"); // not allowed by our versioning scheme
 */
function removeLetters(version: string): string {
    const [$version, prerelease] = version.split("-");
    const prereleaseNumber = prerelease?.replace(/^\D+/g, "");
    const prereleaseString = prereleaseNumber.length
        ? `-${prereleaseNumber}`
        : "";
    return `${$version}${prereleaseString}`;
}

/**
 * @brief Adjusts the versioning convention for Windows devices. This reads the
 * Tauri configuration file, modifies the versioning, and writes it back.
 */
function fixWindowsVersioning() {
    const tauriConfig = JSON.parse(fs.readFileSync(TAURI_CONFIG_PATH, "utf-8"));
    const originalVersion = tauriConfig.version;
    const fixedVersion = removeLetters(originalVersion);
    tauriConfig.version = fixedVersion;
    fs.writeFileSync(TAURI_CONFIG_PATH, JSON.stringify(tauriConfig, null, 4));

    console.log('Torii Version:           ', originalVersion)
    console.log('Torii Version (Windows): ', fixedVersion)
}

// Run the versioning fix.
fixWindowsVersioning();

