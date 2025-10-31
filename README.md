# HNT — Developer Productivity CLI

HNT is a Rust-based Command-Line Interface (CLI) tool that boosts developer productivity by providing modular utilities for **Git automation**, interactive **terminal games**, and **AI integration** — all within a fast, multi-threaded terminal interface. It offers an interactive user interface for selecting options and a set of self-contained commands to streamline common development workflows.

## Table of Contents

- Demo / Screenshots
- Features
- Tech Stack
- Repository Structure
- Requirements
- Installation
- Usage
- Environment Variables / Configuration
- Future Improvements
- Contributing

---

## Demo / Screenshots

### Guessing game

```
$ hnt guess

 (`─').─>           <─. (`─')_ (`─')      (`─')  _   (`─')
 (OO )__      .─>      ╲( OO) )( OO).─>   ( OO).─╱<─.(OO )
,──. ,'─',──.(,──.  ,──./ ,──╱ /    '._  (,──────.,──────,)
│  │ │  ││  │ │(`─')│   ╲ │  │ │'──...__) │  .───'│   /`. '
│  `─'  ││  │ │(OO )│  . '│  │)`──.  .──'(│  '──. │  │_.' │
│  .─.  ││  │ │ │  ╲│  │╲    │    │  │    │  .──' │  .   .'
│  │ │  │╲  '─'(_ .'│  │ ╲   │    │  │    │  `───.│  │╲  ╲
`──' `──' `─────'   `──'  `──'    `──'    `──────'`──' '──'

            🎲 Welcome to the Guessing Game! 🎲

Guess a number between 1-10, or 'q' to quit.
Your guess: 3
Too low! Try again.
Your guess: 6
Too low! Try again.
Your guess: 9
Too high! Try again.
Your guess: 8
🎉 Correct! The secret number was 8.
   You guessed in 4 attempts!
😎 Decent… not bad for a rookie, ngl.
Game over!
```

### Git Push (AI-assisted)

```
$ hnt push -A
◆  Pick a commit message:
│  ● feat: add initial README for HNT CLI tool
│  ○ docs: create README with features, usage, and tech stack
│  ○ chore: create initial README file
│  ○ docs: add README with project description and setup instructions
│  ○ feat: introduce HNT CLI tool with README documentation
│  ○ Custom
└
```

```bash
◇  Pick a commit message:
│  docs: add README with project description
│
└
Using commit message - "docs: add README with project description"
```

AI assisted commit with custom commit msg

```bash
$ hnt push -A
◇  Pick a commit message:
│  Custom
│
│  ○ feat: add initial README for HNT CLI tool
│  ○ docs: create README with features, usage, and tech stack
│  ○ chore: create initial README file
│  ○ docs: add README with project description and setup instructions
│  ○ feat: introduce HNT CLI tool with README documentation
│  ● Custom
◆  Enter your custom commit message:
│  █
└
```

### AI Prompt

```
$ hnt ai "Explain Rust async in simple terms"

Okay, let's break down Rust async in simple terms, focusing on the core concepts and why it's useful.

What's the Problem Async Solves? (The "Why")

Imagine you're a waiter at a busy restaurant...
```

### Scaffold

**Interactive mode**

```bash
$ hnt init
┌  HNT Wizard
│
◆  Current directory is not empty. Please choose how to proceed:
│  ○ Cancel operation
│  ● Remove existing files and continue
│  ○ Igonre files and continue
│
◆  Enter project name:
│  my-project
│
◆  Choose project type:
│  ○ Frontend
│  ○ Backend
│  ● Fullstack
│
◆  Choose frontend framework:
│  ● React
│  ○ Vanilla
│
◆  Choose backend framework:
│  ○ Express.js
│  ● FastAPI (Python)
│  ○ Gin
│  ○ Axum
│
◆  Use TypeScript for frontend?
│  ● Yes  / ○ No
│
◆  Use TailwindCSS?
│  ● Yes  / ○ No
│
└  Scaffolding project...
```

**Inline modifications**

```bash
$ hnt init . --frontend <frontend-lang> --backend <backend-lang>
```

**scaffold directly from config file**

```bash
$ hnt init myapp -y
```

---

## Features

- **Interactive Terminal UI:** Visually appealing menus and prompts with arrow-key navigation, thanks to the Cliclack library, for a smooth CLI experience.
- **Git Automation:** Automate common Git tasks, such as commit, push, setting upstream, and generating commit messages with AI assistance.
- **AI Integration:** Query an AI assistant (using Google Gemini) directly from the terminal; supports full or concise output and easy API key management.
- **Interactive Games:** Built-in terminal games (like a number Guessing Game) for fun or practice without leaving the CLI.
- **Project Scaffolding:** Generate boilerplate for new projects (frontend, backend, or full-stack) using predefined templates (React, Express, FastAPI, Axum, etc.) via an interactive wizard or command-line flags.
- **Configurable and Extensible:** A configuration file (`~/.hnt/config.toml`) stores defaults (e.g., AI key, scaffold preferences). The modular architecture allows adding new commands or plugins easily.

---

## Tech Stack

- **Language:** Rust (for performance and safety).
- **Async Runtime:** Tokio (for multi-threading and async I/O).
- **CLI Parsing & UI:** Clap (command parsing) and Cliclack (interactive terminal interface, arrow-key menus).
- **AI Integration:** Google Gemini API (via a Rust client; requires your API key).
- **Games & Utilities:** Custom game logic and scaffolding generators in Rust.
- **Templates:** Predefined project templates for React (JS/TS), Express (JS/TS), FastAPI (Python), Gin (Go), Axum (Rust), Vanilla JS/TS (with optional Tailwind), etc.

---

## Repository Structure

```
hnt
├── build_release.sh
├── Cargo.lock
├── Cargo.toml
├── install.ps1
├── install.sh
├── LICENSE
├── README.md
├── src
│   ├── ai
│   │   ├── call_ai.rs
│   │   ├── commit.rs
│   │   ├── handler.rs
│   │   ├── mod.rs
│   │   └── update_api_key.rs
│   ├── games
│   │   ├── guess_parser.rs
│   │   ├── guess.rs
│   │   └── mod.rs
│   ├── generator
│   │   ├── axum.rs
│   │   ├── express.rs
│   │   ├── fastapi.rs
│   │   ├── gin.rs
│   │   ├── mod.rs
│   │   ├── react.rs
│   │   └── vanilla.rs
│   ├── git
│   │   ├── mod.rs
│   │   └── push.rs
│   ├── init
│   │   ├── fs_ops.rs
│   │   ├── helper.rs
│   │   ├── mod.rs
│   │   ├── project_summary.rs
│   │   ├── prompts.rs
│   │   ├── scaffold.rs
│   │   ├── validate.rs
│   │   └── wizard.rs
│   ├── main.rs
│   └── utils
│       ├── config.rs
│       ├── loader.rs
│       ├── mod.rs
│       └── pkg_manager.rs
└── templates
    ├── axum
    ├── express
    ├── express-ts
    ├── fastapi
    ├── gin
    ├── react
    │   ├── base
    │   │   └── tailwind
    │   └── tailwind
    ├── react-ts
    │   ├── base
    │   └── tailwind
    ├── vanilla
    │   ├── base
    │   └── tailwind
    └── vanilla-ts
        ├── base
        └── tailwind
```

---

## Requirements

- Rust (latest stable)
- Cargo package manager
- Git (for version control)

---

## Installation

### Requirements

- Rust (latest stable) and Cargo (for development).
- Platform: Linux, macOS, or Windows 10+.

---

### Quick Install (Users)

A convenient install script is provided to download the latest precompiled binary from GitHub Releases:

#### Linux: Run the bash script:

```bash
curl -sSL https://raw.githubusercontent.com/kishore399/hnt/main/install.sh | bash
```

This detects your OS/architecture, downloads the appropriate `hnt` binary, and installs it to `/usr/local/bin`. You may need to enter `sudo` credentials for the move step.

#### Windows: Run the PowerShell script:

```powershell
iwr https://raw.githubusercontent.com/kishore399/hnt/main/install.ps1 -UseBasicParsing | iex
```

This downloads `hnt.exe` and places it in `~/bin` (creating it if necessary) and updates your user PATH.

#### MacOS

For macOS users, it is recommended to build `HNT` from source manually:

```
git clone https://github.com/kishore399/hnt.git
cd hnt
cargo build --release
```

After a successful build, move the binary to your preferred executable directory:

- If Homebrew is installed:
  `sudo mv target/release/hnt /opt/homebrew/bin`

- Otherwise:
  `sudo mv target/release/hnt /usr/local/bin`

This ensures the `hnt` command is available globally in your terminal.

After installation, verify by running:

```bash
hnt --help
```

This should display the help message with available commands.

---

### From Source (Developers)

For development or if you prefer building from source:

Clone repository:

```bash
git clone https://github.com/yourusername/hnt.git
cd hnt
```

Run HNT:

```bash
cargo run
```

Build the project:

```bash
cargo build --release
```

Run the binary:

```bash
./target/release/hnt
```

The binary will be in `target/release/hnt`. You can move it to a directory in your PATH (e.g. `/usr/local/bin`).

---

## Usage

HNT provides several subcommands. Use `hnt --help` or `hnt <command> --help` to see detailed options. Below are common usage patterns:

### Interactive Guessing Game:

```
hnt guess
```

Launch a number guessing game in the terminal. You will be prompted to guess a number between 1 and 10 until correct.

To play non-interactively (single guess):

```
hnt guess <number>
```

Replace `<number>` with your guess (integer). The game will immediately tell you if it’s too high, too low, or correct.

---

### AI Assistant (Google Gemini):

```
hnt ai "<your prompt>"
```

Send a prompt to the AI assistant. By default, it returns a concise answer. Options:

- `-f, --full` : Request the full AI response (long format).
- `-k, --key <API_KEY>` : Set or update your Google Gemini API key.

Example:

```
hnt ai "Explain Rust's ownership model in simple terms"
```

The AI response will be displayed in the terminal. To store your API key (persistently in the config file):

```
hnt ai -k YOUR_API_KEY_HERE
```

---

### Git Automation (Commit & Push):

```
hnt push [options] <commit message> [<branch>]
```

Stages all changes, commits, and pushes to Git. Options:

- `-A, --ai` : Have HNT generate a commit message using AI. You will be prompted to pick from AI-generated suggestions or enter a custom message.
- `-u, --set-upstream` : Push and set the upstream branch on first push (uses `git push --set-upstream origin <branch>`).
- `-n, --dry-run` : Show what would be done without actually committing or pushing.

If `--ai` is not used, provide the commit message as a positional argument. Optionally specify the branch:

```
hnt push -m "Fix issue with API parsing"
```

```
hnt push -u "Initial commit" main
```

With `-A`, e.g.:

```
hnt push -A
```

You will be guided through choosing an AI-generated commit message or writing your own.

---

### Project Scaffold (Init):

```
hnt init [options]
```

Scaffold a new project using templates. If run without options, an interactive wizard will prompt you for project name, type (frontend/backend/fullstack), frameworks (React, Express, FastAPI, etc.), and additional preferences (TypeScript, Tailwind CSS, etc.).

Options to skip the wizard or provide answers directly:

- `-y, --yes` : Answer "yes" to all prompts (uses defaults or values from config).
- `-n, --no` (or `--quick`) : Skip prompts (uses defaults) without confirmation.
- `--frontend <framework>` : Choose frontend framework (`react`, `react-ts`, `vanilla`, `vanilla-ts`).
- `--backend <framework>` : Choose backend framework (`express`, `express-ts`, `fastapi`, `gin`, `axum`).
- `--tailwind` : Include Tailwind CSS in frontend.
- `--git` : Initialize a Git repository.
- `--force` : Overwrite existing files in target directory.

Examples:

```
hnt init
```

```
hnt init -y
```

```
hnt init myapp --frontend react --backend fastapi -y
```

The new project will be created in a subdirectory (or current directory if the name is `.`), with boilerplate code and any necessary setup (e.g. initializing a Git repo).

---

### Configuration Management

HNT stores its configuration and state in `~/.hnt/config.toml`. This file is created on first run and contains settings like:

- **API Keys:** `gemini_api_key` for AI features.
- **Scaffold Defaults:** Default frontend/backend choices, Tailwind usage, and Git initialization.

You can edit `~/.hnt/config.toml` manually or use commands

#### Config Commands

`hnt ai -k <gemini_api_key>`
sets the api key to access ai features in hnt such as ai generated commit messages.

`hnt config set`
Opens an interactive wizard to update scaffold defaults (frontend, backend, Tailwind, Git).
The changes are saved in `~/.hnt/config.toml`.

`hnt config reset`
Resets all configuration values to default.
The old configuration is backed up at `~/.hnt/config.toml.bak`.

Config structure:

```
[api]
gemini_api_key = "<gemini_api_key>"

[init_defaults]
frontend = "<frontend_framework>"
backend = "<backend_framework>"
use_tailwind = <bool>
git_init = <bool>
```

---

### Version Info

```
hnt -V
```

Displays the current HNT version.

---

## Future Improvements

- Plugin system for custom templates
- More terminal games and developer utilities
- Advanced AI features for coding suggestions and documentation
- template fetching from github
- templates managed by open source community

---

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for guidelines on code style, submitting issues, and pull requests.

---

## License

This project is open source under the MIT License. See `LICENSE` for details.

---

## Contact & Credits

Created by **Kishore Kumar (@0xk-h)**.
For any questions or feedback, feel free to open an issue on GitHub or connect on LinkedIn.

GitHub: [https://github.com/0xk-h](https://github.com/0xk-h)

LinkedIn: [https://www.linkedin.com/in/Kishore](https://www.linkedin.com/in/kishore-kumar-547a79329/)

---
