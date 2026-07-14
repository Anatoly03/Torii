Since all contributors would need to start the application anyway, an exhaustive contributors guide is provided in-app after starting in development mode.

This contributors guide is short and will only help you run a minimal working Torii app. First make sure you have the [Rust Programming Language](https://rust-lang.org/) (which comes with [cargo](https://doc.rust-lang.org/cargo/)) and [NodeJS](https://nodejs.org/en) (which comes with [npm](https://docs.npmjs.com/cli/v11/commands/npm) and [npx](https://docs.npmjs.com/cli/v11/commands/npx)) installed. This guide assumes you have some prior familiarity with these tools.

First, install all client dependencies with the following command. You will have to run this command everytime when client dependencies change (e.g. after pulling from remote). This will generate the `node_modules` directory, which contains all client dependencies.

```sh
npm install
```

Afterwards you can start the desktop application with the command below. This command will build the desktop application and watch for changes in the client and desktop codebase to re-run the application.

```sh
npx tauri dev
```

Now you should see the application winow which comes with at least two default workspaces, `Torii Contributors` and `Torii Welcome`. You will find an exhaustive documentation and contributors guide in the first one by simply clicking on it.

<!-- If you are also curious in developing a custom Torii backend, the following command can be used to start the API service.

```sh
cd torii-api
go run src/main.go serve
``` -->
