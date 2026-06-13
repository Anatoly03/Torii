# Torii Project （鳥居Ｐｒｏｊｅｃｔ）

Torii Project is a tool for world builders and story writers to document vast worlds.

### Running

You can serve the application quickly with `npx tauri dev`, which will serve front end, then build aand run the desktop binary.

```sh
npx tauri dev
```

If you work on a headless setup and would like to export environment variables, this command sequence will help. You can for example override the `DESKTOP` environment variable.

```sh
set -a; . .env; set +a
```
