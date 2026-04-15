# GoldenStorm Desktop

GoldenStorm Desktop is the Windows edition of my cross‑platform weather system.  
It delivers a fast, personality‑driven severe weather experience powered by a Rust backend and a lightweight Wry/Tauri‑style UI.

## Features
- ⚡ Real‑time severe weather monitoring  
- 🧠 Personality‑driven weather responses  
- 🖥️ Native Windows desktop experience  
- 🔔 Background agent for continuous alerts  
- 🗂️ Zero‑cost APIs and local‑first configuration  
- 🧹 Clean uninstall with full AppData cleanup  

## Technology Stack
- Rust (async, multi‑threaded backend)
- Wry/Tao desktop runtime
- HTML/CSS/JS UI layer
- NSIS installer with MUI2 branding

## Project Structure
This repository contains the full Windows desktop application, including:
- `GoldenStorm.exe` — main UI  
- `GoldenStormAgent.exe` — background alert agent  
- `assets/` — UI, icons, and state  
- `installer/` — NSIS installer script  
- `src/` — Rust source code  

## Build Instructions
Run the automated build script:


This produces:
- Release binaries in `dist/`
- A versioned installer: `GoldenStormSetup_vX.Y.Z.exe`

## License
This project is currently closed‑source for personal development.
