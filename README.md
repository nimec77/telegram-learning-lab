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


## Lesson 03
Open `lesson-03.html` for media preparation and file boundaries. This self-contained lesson has a synthetic preflight simulator, notes, and embedded Rust starter/reference downloads. The teaching model never opens files, uploads media, or connects to Telegram. `Ready` is not a delivery receipt.

Run `npm test` for parsed-DOM interaction/download checks and `rustc --edition=2021 --test lesson-03-reference.rs -o /tmp/lesson03-tests && /tmp/lesson03-tests` for the ten Rust tests. The starter compiles but deliberately fails until its `todo!` is implemented. Deployment runs both reference tests and expected starter failures before publishing. Sources are pinned to the inspected connector commit.

## Lesson 04
Open `lesson-04.html` to practice routing fictional TeleShelf support commands. A pure Rust policy selects a static draft or reports a reason for human review. The self-contained simulator does not send messages, notify reviewers, or store input. It includes starter and reference downloads.

Run `npm test` for offline simulator and download checks. Run `rustc --edition=2021 --test lesson-04-reference.rs -o /tmp/lesson04-tests && /tmp/lesson04-tests` for the 18 Rust tests. The starter intentionally fails at its `todo!` until implemented.

## Lesson 05
Open `lesson-05.html` for a 95-minute comparison of a fixed TeleShelf support reply and a model-written draft. The exercise keeps review and handoff in Rust, with no send function or Telegram connection; a simulated draft is an offline alternative.

## Lesson 06
Open `lesson-06.html` for a 100-minute exercise using a supplied TeleShelf FAQ. Fictional tickets and typed Rust fixtures separate drafts for review, clarification and handoff. No sending or Telegram connection.
