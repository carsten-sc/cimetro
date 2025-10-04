# Cimetro Setup Guide: VS Code, Rust, and Bevy

This guide sets up **Visual Studio Code (VS Code)**, **Rust**, and **Bevy** for developing *Cimetro*, an open-source city-building simulation, on **Windows** (native) and **Linux via WSL** (cross-platform builds).

## Prerequisites

- **Windows 11**: For native development.
- **WSL2**: For Linux cross-platform builds (Ubuntu recommended).
- **Internet connection**: For downloading tools and dependencies.

## 1. Windows Setup (Native Development)

### Step 1: Install Rust

1. Download and run the Rust installer from [rustup.rs](https://rustup.rs).
2. Choose `stable-x86_64-pc-windows-msvc` (default).
3. Verify in PowerShell:

```powershell
   rustc --version
   cargo --version
```

## Step 2: Install VS Code

1. Download and install VS Code from [code.visualstudio.com](https://code.visualstudio.com).
2. Install these extensions (Ctrl+Shift+X):
   - **rust-analyzer**: Rust code completion and analysis
   - **CodeLLDB**: Rust debugging
   - **dependi**: Helps manage Rust dependencies.
   - **GitLens**: Git integration (optional)
   - **Shader languages support**: For WGSL shaders (wgpu)
   - **WebAssembly**: For modding with WASM
   - **Lua**: For modding with mlua (optional)

---

## Step 3: Install C++ Build Tools

1. Download Visual Studio Build Tools from [visualstudio.microsoft.com/visual-cpp-build-tools](https://visualstudio.microsoft.com/visual-cpp-build-tools).
2. Select the **Desktop development with C++** workload (includes MSVC and Windows SDK for Bevy/wgpu).
3. Complete installation (~5–10 GB).

---

## Step 4: Install Bevy Dependencies

- Ensure **DirectX 12** is available (included in Windows 11).
- Update GPU drivers (NVIDIA/AMD/Intel) from their websites.
- Optional: Install **Vulkan Runtime** for fallback: [vulkan.lunarg.com/sdk/home](https://vulkan.lunarg.com/sdk/home).

---

## Step 5: Create and Test Cimetro

### Create a Bevy project

```powershell
cargo new cimetro
cd cimetro
cargo add bevy
```

### Test a Bevy example

```powershell
cargo run --example 3d_scene
```

### Open in VS Code

```powershell
code .
```

---

## Step 6: Initialize Git for GitHub

1. Install Git for Windows from [git-scm.com/download/win](https://git-scm.com/download/win).
2. Optional: Install [gitdesktop](https://desktop.github.com/download/)
3. Initialize and push to GitHub:

```powershell
git init
git add .
git commit -m "Initial commit for Cimetro"
git remote add origin https://github.com/<your-username>/cimetro.git
git push -u origin main
```

---

## 2. Linux Setup (WSL for Cross-Platform Builds)

### Step 1: Install WSL2

In PowerShell (as Administrator):

```powershell
wsl --install
```

Set WSL2 as default:

```powershell
wsl --set-default-version 2
```

Update Ubuntu in the WSL terminal:

```bash
sudo apt update && sudo apt upgrade
```

---

### Step 2: Install Rust in WSL

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose default option (1) and reload shell:

```bash
source $HOME/.cargo/env
rustc --version
cargo --version
```

---

### Step 3: Install Bevy Dependencies in WSL

Install required packages:

```bash
sudo apt install build-essential pkg-config libssl-dev libx11-dev libxkbcommon-dev libvulkan-dev rsync
```

Optional: Install Vulkan for wgpu:

```bash
sudo apt install libvulkan1 vulkan-tools
```

---

### Step 4: Sync and Build in WSL

Sync the project from Windows to WSL to avoid `/mnt/c` performance issues:

```bash
rsync -av --exclude 'target' /mnt/c/Users/<YourWindowsUsername>/cimetro/ ~/cimetro
```

Build for Linux:

```bash
cd ~/cimetro
cargo build --target x86_64-pc-linux-gnu
```

Optional: Test (if graphical output is available via WSLg):

```bash
cargo run
```

---

### Step 5: Automate Sync and Build (Optional)

Create a script `sync-build.sh`:

```bash
nano ~/sync-build.sh
```

Add:

```bash
#!/bin/bash
rsync -av --exclude 'target' /mnt/c/Users/<YourWindowsUsername>/cimetro/ ~/cimetro
cd ~/cimetro
cargo build --target x86_64-pc-linux-gnu
```

Make executable and run:

```bash
chmod +x ~/sync-build.sh
./sync-build.sh
```

### Summary of Requirements on Windows 11

- **Rust Toolchain:** Install [rustup](https://rustup.rs/) (stable-x86_64-pc-windows-msvc)
- **VS Code:** Install [VS Code](https://code.visualstudio.com/) with extensions (rust-analyzer, CodeLLDB, crates, Shader languages support, GitLens, Lua)
- **Git:** [Install Git for Windows](https://git-scm.com/download/win)
- **C++ Build Tools:** [Install Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with MSVC and Windows SDK
- **DirectX/Vulkan:** Ensure the DirectX runtime is available (default in Windows 11) and optionally [Vulkan Runtime](https://vulkan.lunarg.com/sdk/home)
- **GPU Drivers:** Update NVIDIA/AMD/Intel drivers
- **Lua (optional):** Use mlua for modding (`cargo add mlua`)
- **wgpu (optional):** For GPU AI, use Bevy’s integrated wgpu or add the wgpu crate
- **WSL2 (optional):** Install WSL2 with Ubuntu for Linux builds (`wsl --install`)
- **Cross-Compiling (optional):** Use cross for Linux builds or GitHub Actions for macOS
- **GitHub Desktop (optional):** For easier repository management
- **GitHub Actions:** Set up CI/CD for automated builds

---

## Additional Notes

**Performance Tips:**

- Store the project in Windows (`C:\Users\<YourWindowsUsername>\cimetro`) for native development.
- Use `rsync` to copy to WSL (`~/cimetro`) for Linux builds to avoid slow `/mnt/c` I/O.
- Ensure the WSL VHDX file is on an SSD (`C:\Users\<YourWindowsUsername>\AppData\Local\Docker\wsl\data`).

**Modding and GPU-KI:**

- Add Lua modding: `cargo add mlua`
- Add GPU computing: `cargo add wgpu`

**GitHub and Community:**

- Create a repository: `github.com/<your-username>/cimetro`
- Add a `README.md` with setup, modding info, and MIT license.
- Share on X or itch.io: “Cimetro – Open-Source City Builder with Rust and Bevy!”

**Domain:**

- Register `cimetro.org` (via Namecheap/Cloudflare) for a project landing page.

---

## Next Steps

- Test the setup: Run `cargo run --example 3d_scene` on Windows and WSL.
- Build a prototype: Start with a simple city grid in Bevy.
- Grow the community: Share on X or r/gamedev to attract contributors.

---

For help with a Bevy prototype, README.md template, or setting up cimetro.org, reach out!  
Happy coding with
