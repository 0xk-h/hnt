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

- **Multi-threaded Startup:** Fast module loading using Rust’s asynchronous Tokio runtime, ensuring quick startup even with multiple tools.
- **Interactive Terminal UI:** Visually appealing menus and prompts with arrow-key navigation, thanks to the Cliclack library, for a smooth CLI experience.
- **Git Automation:** Automate common Git tasks, such as commit, push, setting upstream, and generating commit messages with AI assistance.
- **AI Integration:** Query an AI assistant (using Google Gemini) directly from the terminal; supports full or concise output and easy API key management.
- **Interactive Games:** Built-in terminal games (like a number Guessing Game) for fun or practice without leaving the CLI.
- **Project Scaffolding:** Generate boilerplate for new projects (frontend, backend, or full-stack) using predefined templates (React, Express, FastAPI, Axum, etc.) via an interactive wizard or command-line flags.
- **Configurable and Extensible:** A configuration file (`~/.hnt/config.toml`) stores defaults (e.g., AI key, scaffold preferences). The modular architecture allows adding new commands or plugins easily.

---

## Tech Stack

- **Language:** Rust
- **Async Runtime:** Tokio
- **Terminal UI:** Cliclack (arrow keys, interactive menus)
- **AI Integration:** Google gemini
- **Version Control:** Git automation with AI-assisted messages

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

---

## Usage

### Games

To play the interactive Guess game in terminal.

```bash
hnt guess
```

To directly play it without the interactive mode for 1 time

```bash
hnt guess <number>
```

### Git Automation

Automate commits, generate AI-assisted messages, filter commits, and handle upstream configuration.

```bash
# Manual commit
hnt push -m <Commit-msg> <branch>

# Ai generated commit msg (-A or --ai)
hnt push -A or hnt push --ai <bramch>

# Dry-run (-n or --dry-run)
hnt push -An <branch>

# Set upstream in git (-u or --set-upstream)
hnt push -Au main
```

Branch is optional if upstream is already set

### AI Prompts

```bash
hnt ai "Explain Rust async programming"

# -f or --full to see the entire ai response
hnt ai "Explain Rust async programming" -f

# -k or --key to update ai key
hnt ai -k <gemini_api_key>
```

Run prompts directly in the CLI; choose between full or short outputs.

### Project Scaffold

```bash
hnt scaffold fullstack
hnt scaffold frontend
```

Dynamically generate ready-to-run projects with backend/frontend wiring included.

---

## Future Improvements

- Plugin system for custom templates
- More terminal games and developer utilities
- Advanced AI features for coding suggestions and documentation
- template fetching from github
- templates managed by open source community

---

## Contact & Credits

Created by — Kishore Kumar J

GitHub: [https://github.com/0xk-h](https://github.com/0xk-h)

LinkedIn: [https://www.linkedin.com/in/Kishore](https://www.linkedin.com/in/kishore-kumar-547a79329/)

---
