<div align="center">

```
 __    __    __
/ /_  / /_  / /_   ____ - ____  ___  ____ _
/ __ \/ __/ / __/  / __ \  / __ \/ _ \/ __ `/
/ / / / /_  / /_   / /_/ / / /_/ /  __/ /_/ /
/_/ /_/\__/  \__/  / .___/ / .___/\___/\__, /
                  /_/     /_/          /____/
```

# HTTP-REQ

**Minimalist CLI HTTP Client — Built with Rust**

> Stay fast. Stay lean. Say no to Electron.

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![MIT License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-green?style=flat-square)]()
[![CI/CD](https://img.shields.io/badge/CI%2FCD-GitHub%20Actions-black?style=flat-square&logo=github)](https://github.com)

</div>

---

## Why http-req?

Because you deserve a tool that starts in **under 1ms**, not one that boots an entire Chromium engine just to send a `GET` request.

| Feature | http-req | Postman | Insomnia |
|---|---|---|---|
| Startup Time | **< 1ms** | ~3–5s | ~2–4s |
| Memory Usage | **~4MB** | ~500MB+ | ~300MB+ |
| Electron | ❌ Never | ✅ Yes | ✅ Yes |
| Terminal Native | ✅ Yes | ❌ No | ❌ No |
| Zero Login Required | ✅ Yes | ❌ No | ❌ No |

---

## Features

```
⚡  Blazing Fast        Native Rust binary, startup < 1ms
🎨  Smart Output        Auto-detect JSON → pretty print
                        Raw text / HTML → colored output
⏱️  Built-in Benchmark  Shows time elapsed after every request
📦  Zero Bloat          No tracking. No cloud login. Pure terminal.
🛡️  Cross Platform      Linux · Windows (.exe) · macOS
```

---

## Installation

### Option 1 — Install from Source (Recommended)

Make sure you have [Rust installed](https://rustup.rs/) first.

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/tooling.git

# Move into the project directory
cd tooling

# Install globally via Cargo
cargo install --path .
```

### Option 2 — Install Directly via Git

No need to clone manually:

```bash
cargo install --git https://github.com/YOUR_USERNAME/tooling.git
```

### Option 3 — Download Pre-built Binary

Head to the [Releases](https://github.com/YOUR_USERNAME/tooling/releases) page and download the binary for your OS:

| Platform | File |
|---|---|
| Linux | `http-req-linux` |
| Windows | `http-req-windows.exe` |
| macOS | `http-req-macos` |

Then make it executable (Linux/macOS):

```bash
chmod +x http-req-linux
sudo mv http-req-linux /usr/local/bin/http-req
```

---

## Usage

### Basic Syntax

```bash
http-req [METHOD] [URL] [OPTIONS]
```

### Options

| Flag | Description | Example |
|---|---|---|
| `-H` | Add a request header | `-H "Authorization: Bearer token"` |
| `-b` | Set request body | `-b '{"key":"value"}'` |
| `-q` | Add query parameter | `-q "limit=10"` |

---

## Examples

### GET Request

```bash
http-req GET https://api.example.com/v1/products
```

### POST with JSON Body

```bash
http-req POST http://localhost:5126/api/v1/product \
  -H "Content-Type: application/json" \
  -b '{"name":"spotify premium","price":64000}'
```

### GET with Query Parameters

```bash
http-req GET https://api.example.com/search \
  -q "name=rust" \
  -q "limit=10"
```

### GET with Custom Headers

```bash
http-req GET https://api.example.com/profile \
  -H "Authorization: Bearer your_token_here" \
  -H "Accept: application/json"
```

### POST with Auth Header + Body

```bash
http-req POST https://api.example.com/login \
  -H "Content-Type: application/json" \
  -b '{"email":"user@example.com","password":"secret123"}'
```

---

## Sample Output

```
> GET https://api.example.com/v1/products

HTTP/1.1 200 OK
Content-Type: application/json

{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Spotify Premium",
      "price": 64000
    }
  ]
}

⏱  Time elapsed: 142ms
```

---

## Project Architecture

```
src/
├── main.rs          → Orchestrator + timing logic
├── cli.rs           → CLI definition (Clap)
└── http/
    ├── request.rs   → Request builder (Reqwest)
    └── response.rs  → Response formatter + pretty print
```

The project follows a **Vertical Slice Architecture** — each concern is cleanly separated and self-contained.

---

## CI/CD Pipeline

Powered by **GitHub Actions**:

```
On every push:
  ✅ cargo check
  ✅ cargo test

On git tag push (e.g. v0.1.0):
  📦 Build binary → Linux
  📦 Build binary → Windows (.exe)
  📦 Build binary → macOS
  🚀 Publish to GitHub Releases
```

---

## License

MIT © 2026 — Created by **awn**

> *Built for developers who respect their machine.*

---

<div align="center">

**[⭐ Star this repo](https://github.com/YOUR_USERNAME/tooling)** if it saves you from opening Postman ever again.

</div>
