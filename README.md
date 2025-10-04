# 📚 Praxis - Turning Theory into Practice

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Hacktoberfest](https://img.shields.io/badge/Hacktoberfest-2025-orange)](https://hacktoberfest.com/)

> **Praxis** is an open-source, Rust-powered interactive learning platform that bridges the gap between theory and practice. Built by the community, for the community - where learners master programming languages through hands-on exercises and contributors share their expertise.

## ✨ Features

- 🚀 **One-Command Setup**: Install the Praxis runner (`px`) with a single command - it handles test execution and progress tracking automatically
- 🤝 **Community-Driven**: Contributors can easily add test cases, exercises, and theory content for any programming language
- 📖 **Auto-Generated Documentation**: Beautiful, organized docs generated from Markdown files using Static Site Generation
- 💻 **Terminal-Based Learning**: Learn directly in your preferred development environment
- 🎯 **Test-Driven Approach**: Practice with real test cases that validate your understanding Turning theory into Practice

`Praxis` is an open-source, Rust-powered interactive learning platform where anyone can contribute their experience. It’s designed for learners who want to pick up a programming language through hands-on practice and quick access to theory.


## Features

* **Easy setup:** Single command to install a Praxis runner `(px)` which checks the test cases and saves the user progress
* **Community Contributions:** Contributors can easily add new test cases and theory content for any programming language.
* **Static Site Generated Docs:** Documentation is auto-generated from Markdown files using SSG providers — ensuring content stays organized and accessible.


## 🌍 Our Vision

We're building a **living knowledge base** where the global developer community collaborates to create test-driven exercises and comprehensive theory for different programming languages. 

Think of Praxis as:
- 📚 An interactive textbook that evolves with the community
- 🧪 A coding laboratory in your terminal
- 🌱 A platform where knowledge grows through shared experiences

## 🏗️ Project Structure

Here's how Praxis is organized:

```
praxis/
├── 🦀 px-runner/              # Rust-based test runner
│   ├── src/
│   └── Cargo.toml
├── 📝 exercises/              # Programming challenges
│   ├── python/
│   │   ├── add.py
│   │   ├── test_add.py
│   │   ├── palindrome.py
│   │   └── test_palindrome.py
│   └── javascript/
│       └── ...
├── 🌐 docs/                   # Static site generation
│   └── zola/
├── Cargo.toml
└── README.md
```
## 🚀 Quick Start

### Installation (Coming Soon!)

We're working on a simple installation process. Soon you'll be able to install Praxis with:

```bash
curl -sSL https://install.praxis.dev | sh
```

This will download the latest release and install the `px` runner to your system.

### 📚 Learning a Language

Choose your learning path by cloning a specific language branch:

```bash
# Learn Python
git clone --branch python https://github.com/Open-Source-Chandigarh/praxis.git praxis-python

# Learn JavaScript (coming soon)
git clone --branch javascript https://github.com/Open-Source-Chandigarh/praxis.git praxis-js
```

### 🏃‍♂️ Start Practicing

Navigate to your downloaded folder and start learning:

```bash
cd praxis-python
px start  # Launch the Praxis runner
```

The `px` runner will:
- ✅ Execute test cases automatically
- 📊 Track your learning progress  
- 💡 Provide instant feedback
- 🎯 Guide you through exercises

## 🤝 Contributing

We love contributions! Whether you're fixing bugs, adding new exercises, or improving documentation - every contribution makes Praxis better.

### Ways to Contribute:
- 🐛 **Bug Reports**: Found an issue? Let us know!
- ✨ **New Features**: Have an idea? We'd love to hear it!
- 📝 **Exercises**: Add challenges for any programming language
- 📚 **Documentation**: Help make our docs clearer and more comprehensive
- 🌍 **Translations**: Make Praxis accessible worldwide

**Ready to contribute?** Check out our [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.


## 💬 Community

Join our growing community of learners and contributors:

- 💬 **Discussions**: Share ideas and ask questions in [GitHub Discussions](https://github.com/Open-Source-Chandigarh/praxis/discussions)
- 🐛 **Issues**: Report bugs or request features in [GitHub Issues](https://github.com/Open-Source-Chandigarh/praxis/issues)
- 🌟 **Star the repo**: Show your support and help others discover Praxis

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](./LICENSE) file for details.

---

<div align="center">

**Made with ❤️ by the [Open Source Chandigarh](https://github.com/Open-Source-Chandigarh) community**

*Praxis: Where theory meets practice, and learning never stops* 🚀

</div>
