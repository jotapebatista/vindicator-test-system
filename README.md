<h1 align="center">BARIX - Vindicator Test System</h1>
<br />

<p float="left">
	<img src="https://img.shields.io/github/package-json/v/jotapebatista/vindicator-test-system" />
	<img src="https://img.shields.io/github/license/jotapebatista/vindicator-test-system" />
</p>


## Technologies run-down

- Nuxt 3
- Tauri 2
- UnoCSS
- Typescript
- ESLint
- Auto imports (for Tauri api too!)

## Functionalities

- Runs a test wizard to test the Barix Vindicator (LED Strobe Device)

## Setup
  - This project enforces [pnpm](https://pnpm.io). In order to use another package manager you need to update `package.json` and `tauri.conf.json`
  - The frontend runs on the usual port `3000` of Nuxt, the Tauri server uses the port `3001`. This settings are customizable in the `nuxt.config.ts` and `tauri.conf.json`.
  - Once ready, follow these commands:

  ```sh
  # go into the folder
  $ cd my-nuxtor-app

  # install dependencies
  $ pnpm install

  # start the project
  $ pnpm run tauri:dev
  ```

  This will run the Nuxt frontend and will launch the Tauri window.

## Build

  ```sh
  $ pnpm run tauri:build
  ```

This command will generate the Nuxt static output and bundle the project under `src-tauri/target`.

## Debug

  ```sh
  $ pnpm run tauri:build:debug
  ```

The same Tauri bundle will generate under `src-tauri/target`, but with the ability to open the console.
