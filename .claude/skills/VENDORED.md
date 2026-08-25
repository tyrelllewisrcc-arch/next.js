# Vendored third-party skills

The skills listed here were copied in from public GitHub repositories. They are
**not** part of the Next.js project and are not maintained by it. Everything
under `.claude/skills/` that is not listed in this file is first-party (see
`.agents/skills/README.md` for authoring guidance on repo-owned skills).

Copied on 2026-08-25. Each entry pins the upstream commit that was vendored so
the diff can be reproduced or refreshed.

## Sources

| Source | Commit | License | Skills installed |
| --- | --- | --- | --- |
| [vercel-labs/skills](https://github.com/vercel-labs/skills) | `435076e` | MIT (© 2026 Vercel, Inc.) | `find-skills` |
| [obra/superpowers](https://github.com/obra/superpowers) | `b36e082` | MIT (© 2025 Jesse Vincent) | 14 skills, see below |
| [rebelytics/one-skill-to-rule-them-all](https://github.com/rebelytics/one-skill-to-rule-them-all) | `281f134` | CC BY 4.0 (© Eoghan Henn / rebelytics.com) | `task-observer` |
| [pbakaus/impeccable](https://github.com/pbakaus/impeccable) | `fcd7622` | Apache 2.0 | `impeccable` (docs only — see caveat) |
| [thedotmack/claude-mem](https://github.com/thedotmack/claude-mem) | `e2d1df5` | Apache 2.0 | 9 skills, see below |

### obra/superpowers (14)

`brainstorming`, `dispatching-parallel-agents`, `executing-plans`,
`finishing-a-development-branch`, `receiving-code-review`,
`requesting-code-review`, `subagent-driven-development`,
`systematic-debugging`, `test-driven-development`, `using-git-worktrees`,
`using-superpowers`, `verification-before-completion`, `writing-plans`,
`writing-skills`

Vendored as plain skills. The upstream project also ships a `SessionStart` hook
and a Claude Code plugin; neither was installed. Upstream's optional
brainstorming "visual companion" loads a logo from the author's site as install
telemetry — set `SUPERPOWERS_DISABLE_TELEMETRY=1` to opt out. Nothing here
phones home on its own.

### thedotmack/claude-mem (9)

`babysit`, `design-is`, `do`, `learn-codebase`, `make-plan`, `oh-my-issues`,
`pathfinder`, `what-the`, `wowerpoint`

claude-mem is a memory-compression **application** (npm package, CLI, MCP
server, SQLite store, session hooks), not a skill bundle. Only the 9 skills that
work without that runtime were vendored. The other 10 upstream skills
(`mem-search`, `cloud-sync`, `weekly-digests`, `timeline-report`, `standup`,
`knowledge-agent`, `smart-explore`, `mode-creator`, `how-it-works`,
`version-bump`) are front-ends for the claude-mem daemon and would be broken
without it. To get those, install the real thing at the user level:
`npx claude-mem install`.

## Caveats

- **`impeccable` is docs-only.** `SKILL.md` and `reference/` are installed, but
  the upstream `scripts/` directory (107 `.mjs`/`.js` files — detector rules, a
  live browser server, pre-edit hooks) was not. The skill's setup step calls
  `node <skill-base-dir>/scripts/context.mjs`, which will fail as installed. For
  a working install, run the upstream installer from the project root:
  `npx impeccable install`.
- **`babysit` shadows a convention name.** Some harnesses look for
  `.claude/skills/babysit/SKILL.md` as *repo-specific* PR guidance. The file here
  is claude-mem's generic PR-watching skill, not a Next.js policy document.
  Rename or remove it if that ambiguity matters.
- **Skill descriptions cost context every session.** These add ~25 skills whose
  descriptions load into every session. Prune the ones you don't use.
- **`using-superpowers` and `task-observer` are assertive by design.** Both ask
  to be invoked at the start of every session. That is upstream's intent, not a
  repo requirement.

## Updating

Re-clone the source at a newer commit and re-copy the skill directories, then
update the commit column above. There is no automated sync.
