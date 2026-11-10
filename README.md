![Dress](resources/dress-small.png)

---

# Dress

A utility for analyzing a set of instructions or a recipe and determine the "dressings" or ingredients needed in order of need.

Dress is inspired by Shazow's backless philosophy for apps, or at least my interpretation of it.

A few notes:

* Unlike most of my apps, this one doesn't use a config file. Everything is either defaults or arguments.
* When sourcing from a website, cert verification is disabled. As the goal is pulling a known source, cert verification makes no sense.
* In the initial version, analysis will be done with Ollama compatible self hosted.

---

## Usage

Usage: dress [(OPTIONS)]

  A utility to find the dressings.

Options:

  --mode <MODE>                        Mode to run in (default: cli, choices: cli, web, help)
  --llm_server                         LLM server to interpret  the text, ollama compatible, (default: http://localhost:11434/api/generate)
  --model                              LLM model to use to interpret the text (default: gemma3:4b-it-qat)
  --listen                             In web mode, address and port to listen on (default: 0.0.0.0:13874)
  --source                             The text source, which can be a URL or a file path, no default.
  --help                               Display this help message

---

## Reference:

* [Changelog](CHANGELOG.md)
* [To Do List](TODO.md)

### External

* [Shazow](https://shazow.net/)

---

©2026 Bethany Davis <ellis@despoena.com>.
