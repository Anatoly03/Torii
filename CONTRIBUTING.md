Since all contributors would need to start the application anyway, an exhaustive contributors guide is provided in-app after starting in development mode.

If you are serious about contributing to this repository, you can find a quick way to start the application in development mode below. Please be aware of the [Pull Request Rules](#pull-request-rules). Breaking these rules will result in reactions such as your Pull Request to be rejected or your account restricted in this repository.

### Quickstart

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

### Pull Request Rules

- **No vibes.** You are allowed to use smart, contextless autocompletion and ask smart assistants for planning and code review. However code which has been subject to agent generation is categorically not allowed in this repository.

- **English Codebase.** We allow to interact with this repositories issuesses, discussions and comments in different languages. However excluded are all commit messages, source code and the title and first message in a Pull Request, which have to be in English.

- **No obfuscastions.** Code using obfuscations for the sake of being unreadable is not allowed. Performance-optimized code has to be clearly documented with comments.

- **Citations.** Code retrieved from external websites like StackOverflow have to be cited in the codebase. A citation is a comment with a link above the cited code segment. Links that helped you fix bugs or helped you choose an algorithm or approach are welcomed.

- **Documentation.** All code is subject to exhaustive documentation. Make sure the formatting is not broken for documentation generators. For functions include example and usage. We try to keep a 1:1 to 1:2 (code lines:documentation) ratio.

- **Rules are non-exhaustive.** The administrative team of this codebase reserves the right to make up new written rules when "unspoken rules" are broken. The "unspoken rules" include: Be Nice, Keep the Code Clean, Follow Existing Patterns or Evolve New Maintainable Patterns.
