/**
 * @brief In previous publish builds, the versioning for pre-releases contained a string. We
 * intend to keep this string. On Windows however, the versioning is expected to be a number
 * @see https://github.com/Anatoly03/Torii/actions/runs/27463818799/job/81182445945
 */

/**
 * @brief Path to the Tauri configuration file.
 */
// TODO
const TAURI_CONFIG_PATH = "./tauri.conf.json";

/**
 * @param version Version provided by the repository in `tauri.conf.json`.
 * @return Version string with non-leading letters removed.
 * @example
 *
 * assert.equal(removeLetters("0.0.0-dev5"), "0.0.0-5");
 * assert.equal(removeLetters("0.0.1"), "0.0.1");
 * assert.equal(removeLetters("0.1.0-alpha"), "0.1.0"); // not allowed by our versioning scheme
 */
function removeLetters(version: string): string {
    const [version, prerelease] = version.split("-");
    const prereleaseNumber = prerelease?.replace(/^\D+/g, "");
    const prereleaseString = prereleaseNumber.length
        ? `-${prereleaseNumber}`
        : "";
    return `${version}${prereleaseString}`;
}

// TODO add logic when building for windows

// - name: Adjust versioning convention for Windows
//   if: matrix.platform == 'windows-latest'
//   run: ts-node .github/scripts/fix-windows-versioning.ts
