# konslo

A minimalist habit tracking app built with a Rust backend and Angular frontend.

## About

This is a personal project built to practice full-stack development with a modern, performance-oriented stack. 
The goal is simple: track daily habits, stay consistent, build momentum.
I chose Rust for the backend to push myself beyond typical Node/Python stacks — and to learn what it really means to write safe, concurrent, and blazing-fast server code.

### Features

1. [X] Create and manage daily habits 
2. [ ] Mark habits as done for the day 
3. [ ] View your current streak 
4. [ ] Stats & progress charts 
5. [ ] User authentication 
6. [ ] Make `serde` an optional feature in `konslo-core` to keep the domain layer pure by default


### What I want to Learn

Setting up a REST API in Rust with Actix-web — the ownership model forced me to think differently about data flow
Structuring an Angular app with services, reactive forms, and component communication
Designing a clean API contract between two completely separate tech stacks
Managing CORS, async handlers, and error types in a strongly-typed language

### Version

v1 — CRUD habits + frontend Angular. C'est ton MVP, tu valides que tout fonctionne bout en bout. 

v2 — Completions. Un habit peut être complété chaque jour. Tu ajoutes juste une table completions avec une foreign key vers habits. Pas d'auth, pas de complexité.

v3 — Statistiques. Streaks, taux de complétion sur 7/30 jours, visualisation dans Angular. Pure lecture, pas de nouveaux domaines.

v4 — Users + Auth JWT. Tu rattaches habits et completions à un utilisateur. C'est le gros chantier — middleware auth, refresh tokens, etc.

v5 — Multi-device, notifications, rappels.

## 🤝 Prompt Assistance — Claude (MyFav <3)

This project was built in collaboration with Claude (Anthropic) as a learning exercise.

### Our workflow

- **Small steps** — we move forward one step at a time. No long "step 1, 2, 3, 4" plans upfront. First a short description of the approach, then one message per step.
- **Intuition first** — before giving a solution, Claude challenges my intuition: *"what do you think...?"* rather than handing everything over.
- **RemNote flashcards** — every time a major new concept is introduced, Claude produces a small flashcard in English to integrate into my study notes.
- **Understanding before copy-pasting** — we always take the time to understand *why* before writing any code.
- **Atomic git commits** — after each meaningful implementation step, we stop and commit with a clear, focused message. One concept = one commit.
- **TDD** — we write the test first, watch it fail (red), then write the code to make it pass (green), then clean up (refactor).
- **PowerShell first** — on utilise les commandes PowerShell autant que possible (ni, rm, mv, cat, etc.)
- **IntelliJ tips** — de temps en temps, Claude glisse un tip sur l'utilisation d'IntelliJ pour aller plus vite.

### Flashcard format example

```
Flashcard 📇
  syn crate in Rust
  Parses Rust source code into an AST (Abstract Syntax Tree)
  Lets you extract fn signatures, structs, traits, impls without manual text parsing
  Used by the Rust compiler toolchain itself
  Common pattern: syn::parse_file() → visit items → match on syn::Item variants
```