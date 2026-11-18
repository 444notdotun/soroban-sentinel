# Open Source Contribution Plan & Drips Wave Backlog

**Project:** `soroban-sentinel`  
**Architecture:** Modular Monolith (2 Crates: `sentinel-core` and `sentinel-cli`)  
**Program:** Stellar Wave on Drips Network  
**Maintainer:** Lead Project Architect  
**Repository Policy:** Public, MIT Licensed, Continuous Integration Gated  

---

## 1. Drips Wave Program Alignment & Backlog Overview

The **Stellar Wave Program on Drips** brings external developers into high-impact Stellar repositories during week-long contribution sprints. To ensure high-quality contributions, prevent maintainer bottlenecks, and maximize reward fairness, issues are curated and weighted according to standard Wave complexity points:

* **Trivial (100 Points)**: Well-defined tasks, CLI formatting flags, domain math unit tests, parameter validation. Ideal for first-time contributors.
* **Medium (150 Points)**: XDR key derivation, RPC chunking and retry middleware, in-process alert handlers, Prometheus metric collectors.
* **High (200 Points)**: Complex cryptographic transaction assembly, rent simulation calculators, keeper safety bounds. Requires deep protocol familiarity.

---

## 2. Contributor Workflow & Standards

### Step-by-Step Workflow
1. **Find an Issue**: Browse open GitHub issues using the filters in [Section 3](#3-finding-work). Review the description, scope, acceptance criteria, and `complexity:` label.
2. **Apply & Assign**: Apply for the task via the Drips Wave portal (or comment on the GitHub issue). The maintainer will review your profile and assign you to the issue.
3. **Fork & Branch**: Create a feature branch off `main`:
   ```bash
   git checkout -b feature/issue-<number>-<short-description>
   ```
4. **Develop & Verify**: Ensure all changes conform to repository quality standards:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
5. **Submit PR**: Open a Pull Request referencing the issue (e.g., `Closes #<number>`). Include a brief description of the implementation and test verification.
6. **Review & Merge**: Maintainers review PRs within 24–48 hours. Once merged, maintainers resolve the issue in Drips so rewards are distributed automatically.

---

## 3. Finding Work

[GitHub Issues](https://github.com/ekwe7/soroban-sentinel/issues) is the single source of truth for available work. This document intentionally does not list individual issues, so it never goes stale as issues are opened, renamed, or closed.

### Useful filters
* [Open issues accepting contributors](https://github.com/ekwe7/soroban-sentinel/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22+no%3Aassignee): `help wanted`, not yet assigned.
* [High-complexity issues](https://github.com/ekwe7/soroban-sentinel/issues?q=is%3Aissue+is%3Aopen+label%3A%22complexity%3A+high%22): `complexity: high`.
* [Good first issues](https://github.com/ekwe7/soroban-sentinel/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22): `good first issue`, when available.

### Reading an issue
Every curated issue follows the same structure: **Description**, **Scope** (including what is out of scope), **Acceptance criteria**, **Implementation notes**, and **Verification**. If an issue builds on another, its body starts with a `Depends on:` line; do not start it until the dependency is merged.

### Coordinating with other contributors
Several features touch the same CLI entry points (`crates/sentinel-cli/src/args.rs` and `crates/sentinel-cli/src/commands/scan.rs`). Keep changes to those files minimal and rebase on `main` before requesting review.

### Where to look for context
* Delivered functionality: [Release Notes v0.1.0](RELEASE_NOTES_v0.1.0.md).
* Planned direction and phase ordering: [Implementation Roadmap](IMPLEMENTATION_ROADMAP.md).
* Proposing new work: open a GitHub issue describing the problem before writing code, so scope can be agreed first.
