# Telegram Learning Lab

A public, self-contained learning site about safe Telegram tools and applied AI. Lesson 01 covers MCP vs Telegram, tracing an existing read operation, an offline example and a proposed safe text-send contract.

## Use offline
Open `index.html` in any modern browser. No install or build is required. External readings and video require internet. Progress and notes use browser-local storage when available; export notes for a durable copy. There is no backend, analytics or Telegram connection.

## Publish on GitHub Pages
Create a dedicated public repository, upload these files and set **Settings → Pages → Build and deployment → Source: GitHub Actions**. The included workflow deploys `index.html` when pushed to `main` or manually dispatched. It does not touch the Telegram Connector repository.

## Future lessons
Add self-contained lesson HTML files and link them from the course roadmap. Do not publish credentials, account sessions, private conversations or personal notes.

## Validation
Run `python3 validate.py` for basic structural and privacy checks. Browser QA is also needed for responsive layout and interactions.

## Lesson 02
Open `lesson-02.html` for the offline safe text-send lab. It includes starter and reference Rust downloads, eight tests, an interactive outcome model and pinned source comparisons. `reference.rs` is a teaching model, not a Telegram client. Run `npm ci` once, then `npm test` for HTML-parsed DOM interaction checks (including selectable modes, receipts and lost responses) and `rustc --edition=2021 --test reference.rs -o /tmp/lesson-tests && /tmp/lesson-tests` for Rust tests.
