# iloader MemoryPlus (non ufficiale)

> **Questa NON è la versione ufficiale di iloader.** È un fork di [iloader](https://github.com/nab138/iloader) di [nab138](https://github.com/nab138), non affiliato né approvato dall'autore originale. L'iloader ufficiale si scarica solo da [iloader.app](https://iloader.app) o da [github.com/nab138/iloader](https://github.com/nab138/iloader).
>
> *Unofficial fork of iloader that requests the "Increased Memory Limit" capability for every installed app. Not affiliated with or endorsed by the original author.*

## Cosa cambia rispetto all'originale

Solo queste cose, tutte visibili nella cronologia dei commit:

1. **Increased Memory Limit (Memory+) sempre attivo.** Quando installi un'app con *Importa IPA*, iloader aggiunge all'App ID la capability "Increased Memory Limit". Serve ad app come [Madeira](https://github.com/willfaust/Madeira), che senza Memory+ non riescono ad avviare i giochi. Nel codice è una riga: `false` → `true` in `src-tauri/src/sideload.rs`.
2. **Aggiornamento automatico disattivato.** L'iloader ufficiale si aggiorna da solo alla versione ufficiale, e così Memory+ andrebbe perso.
3. **Nome e identificativo diversi** (`iloader MemoryPlus`, `app.iloadermemoryplus`), così non si confonde con l'iloader ufficiale e può stare installato accanto a lui.
4. **Compilazione automatica** con GitHub Actions: gli exe nella pagina Releases sono compilati dal codice di questo repository, non caricati a mano.

Tutto il resto (accesso Apple ID, firma, installazione) è identico all'iloader ufficiale su cui è basato (**v2.3.10**).

## Come si usa

1. Installa [iTunes](https://apple.co/ms) (dal sito Apple, non dal Microsoft Store).
2. Scarica l'ultima versione dalla pagina **Releases** di questo repository e controlla l'hash con `SHA256SUMS.txt`.
3. Collega l'iPhone, apri iloader MemoryPlus e accedi con il tuo Apple ID (meglio uno secondario).
4. Se l'accesso dà l'errore **-80045**, nelle impostazioni scegli un server anisette **Windows**.
5. Usa **Importa IPA** per installare l'app: Memory+ viene aggiunto da solo.

Windows potrebbe mostrare "PC protetto da Windows" perché l'exe non è firmato: *Ulteriori informazioni → Esegui comunque*.

## Avvertenze

- Usalo a tuo rischio: gestisce le credenziali del tuo Apple ID, come ogni sideloader. Il codice è tutto qui, controllalo.
- Per problemi che **non** riguardano Memory+ fai riferimento all'iloader ufficiale. Per problemi di questo fork apri una issue **qui**, non sul repository originale.

---

# README originale di iloader

## How to use

- Install usbmuxd for your platform
  - Windows: [iTunes](https://apple.co/ms)
  - macOS: Included
  - Linux: Potentially included, if not, install via your package manager
- Install the latest version for your platform from the [releases](https://github.com/nab138/iloader/releases)
  - NixOS: Use the flake `github:nab138/iloader`
  - Fedora: Use the .RPM or the unofficial [Fedora COPR repository](https://copr.fedorainfracloud.org/coprs/anudeepd/iloader)
  - Arch: Use the unofficial [AUR package](https://aur.archlinux.org/packages/iloader-bin),
- Plug in your iDevice to your computer
- Open the app
- Sign into your Apple ID
- Select your action (e.g. install SideStore)

## Features

- Install SideStore (or LiveContainer + SideStore), import certificate and place rppairing+lockdown pairing files automatically
- Import any IPA
- Intelligent error suggestions to help resolve common issues
- Manage pairing files in apps like StikDebug, SideStore, Protokolle, etc
- See and revoke development certificates & app ids

## Troubleshooting

- If you are unable to solve an issue on your own, copy the full error message and ask on the [idevice Discord server](https://discord.gg/EA6yVgydBz) or [open an issue](https://github.com/nab138/iloader/issues).
- You can view app logs with the "View Logs." If nothing is showing up, change the log level to "Debug."
- If those logs aren't helpful, logs with additional are stored in the following locations:
  - Windows: `%APPDATA%\me.nabdev.iloader\logs`
  - macOS: `~/Library/Application Support/me.nabdev.iloader/logs`
  - Linux: `~/.local/share/me.nabdev.iloader/logs/`

## Translating

iloader needs localization! If you speak another language and notice iloader does not support it or has mistakes, please consider contributing.

To update/edit an existing language, make a PR modifying `src/locales/<lang>.json`.

To add a new language, add your language to `src/i18next.ts`, and in `src/locales` copy `en.json` to a new file titled `<langcode>.json` and update the strings.

**i18next.ts:**

```ts
const languages = [
  ["en", "English"],
  ["es", "Español"],
  // Your language here...
] as const;
```

You can also add your name to the translators section of the README.

Thank you for translating!

## Building from source

1. Install [bun](https://bun.sh) (or [Node.js](https://nodejs.org)) and [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository and `cd` into it
3. Run `bun i` (or `npm i`)

For development with hot reload: `bun tauri dev` (or `npm run tauri dev`)
Make a production build: `bun tauri build` (or `npm run tauri build`)

## Credits

- Icon made by [Transistor](https://github.com/transistor-exe)
- UI improved by [StephenDev0](https://github.com/StephenDev0)
- [idevice](https://github.com/jkcoxson/idevice) by [jkcoxson](https://github.com/jkcoxson) for communicating with iOS devices
- [isideload](https://github.com/nab138/isideload) for installing apps
  - [idevice](https://github.com/jkcoxson/idevice) by [jkcoxson](https://github.com/jkcoxson) crate is used to communicate with the device
  - [apple-codesign-quick](https://github.com/Dadoum/apple-codesign-quick) by [Dadoum](https://github.com/Dadoum) for codesigning and entitlements
  - [Impactor](https://github.com/claration/Impactor) by [claration](https://github.com/claration) was used as a reference for cryptography operations (converting certs to p12, etc.).
  - [Sideloader](https://github.com/Dadoum/Sideloader) by [Dadoum](https://github.com/Dadoum) was used as a reference for how apple private developer endpoints work
- [idevice_pair](https://github.com/jkcoxson/idevice_pair) was used as a reference for pairing file management
- App made with [tauri](https://tauri.app)

## Translators

Thank you to everyone who has contributed translations! See the [Translating](#translating) section if you would like to contribute as well.

- [By3lish](https://github.com/by3lish): Azerbaijani (az)
- [TNT-333](https://github.com/TNT-333): German (de)
- [basketshoe](https://github.com/basketshoe): Italian (it)
- [baocreata](https://github.com/baocreata): Vietnamese (vt)
- [IamArayel](https://github.com/IamArayel): French (fr)
- [kkula9999](https://github.com/kkula9999): Traditional & Simplified Chinese (zh_tw & zh_cn)
- [sibwaze](https://github.com/sibwaze): Russian (ru)
- [notmalicik](https://github.com/notmalicik): Română (ro)
- [mirdukkkkk](https://github.com/mirdukkkkk): Improved Russian (ru)
- [okinaau](https://github.com/okinaau): Arabic (ar)
- [ChouChiu](https://github.com/ChouChiu): Cantonese (zh_hk) & Improved Chinese (zh_tw & zh_cn)
- [marcinmajsc](https://github.com/marcinmajsc): Polish (pl)
- [ern775](https://github.com/ern775): Turkish (tr)
- [canpng](https://github.com/canpng): Improved Turkish (tr)
- [jazoppix](https://github.com/jazoppix): Spanish (es)
- [eseiker](https://github.com/eseiker): Korean (ko)
- [seomin0610](https://github.com/seomin0610): Improved Korean (ko)
- [Ordyan777](https://github.com/Ordyan777): Armenian (am)
- [kakik0u](https://github.com/kakik0u): Japanese (ja)
- [lkspodmol](https://github.com/lkspodmol): Czech (cs_cz)
- [marcusherelammonstyle-cmd](https://github.com/marcusherelammonstyle-cmd): Swedish (sv)
- [MCI49312](https://github.com/MCI49312): Hungarian (hu)
- [Kynonim](https://github.com/Kynonim): Indonesian (id)
- [DD00031](https://github.com/DD00031): Dutch (nl)
- [Toritan123](https://github.com/Toritan123): Improved Japanese (ja)
- [marcinmajsc](https://github.com/marcinmajsc): Improved Polish (pl)
- [dleiferives](https://github.com/dleiferives): Greek (el)
- [ShadowWLX](https://github.com/ShadowWLX): Improved French (fr)
- [fkpcomposer](https://github.com/fkpcomposer): Brazilian Portuguese (pt_br)
- [474FrediFred](https://github.com/474FrediFred): Swiss German (de_ch)
- [bovae](https://github.com/bovae) & [b1twalker](https://github.com/b1twalker): Ukranian (uk)

## License

Copyright (C) 2026 nab138

The source code of this repository is licensed under the MIT License. See the [LICENSE](/LICENSE) file for the full text.

Branding, logos, media assets, and the name “iloader” are not licensed under the MIT License and are subject to separate restrictions.

You may retain or use branding materials in forks, tutorials, or documentation if you include a clear link to either the official site (https://iloader.app) or the iloader source code repository (https://github.com/nab138/iloader) and do not imply official endorsement. See [LICENSE-BRANDING](/LICENSE-BRANDING) for full details.

## Contributing

Please read our [Contributing](/CONTRIBUTING.md) policy before making a contribution.

## Future Plans

iloader is currently being overhauled, see the [trello board](https://trello.com/b/hAe1LHTj/iloader) for more info!
