---
name: agent-fanout
description: >
  How to split work across parallel subagents and synthesize their reports
  (fan-out / fan-in). Use when a task spans several independent files, packages,
  test suites, CI jobs, or research threads, or when the user asks to
  "use agents", "fan out", "run these in parallel", "delegate", or "spawn
  subagents". Covers choosing between inline work, a forked subagent, and a cold
  subagent, partitioning without overlap or seams, writing prompts for agents
  that inherit no conversation history, enforcing read-only with tool scoping
  rather than prose, and reconciling reports that conflict.
---

# Agent Fan-Out / Fan-In

Use this skill when deciding whether to split work across subagents, and when
writing the worker prompts and synthesizing what comes back.

The value of this pattern is not "spawn agents". It is matching the shape of the
execution to the shape of the work. A cold subagent inherits none of the parent's
conversation history and must re-derive context from scratch, so fan-out applied
to the wrong task is strictly worse than doing it inline — slower, more
expensive, and more likely to return confident nonsense. Token cost is
multiplicative across agents. Decide first, then delegate.

## Pick the Primitive

Four options, not two. Most bad fan-outs are really a wrong choice here.

| Situation                                                          | Use                                                                  |
| ------------------------------------------------------------------ | -------------------------------------------------------------------- |
| You already have the context; the change is small or one grep away | Do it inline                                                         |
| Side task needs your full context but shouldn't pollute it         | Forked subagent (`context: fork`) — inherits the parent conversation |
| Independent, self-contained work over a surface you have not read  | Cold subagent — the actual fan-out                                   |
| An agent already did adjacent work and you need more from it       | `SendMessage` to resume it; it keeps its full history                |

Spawning a fresh Agent always starts cold. Resuming is usually cheaper and better
informed than re-spawning.

## Decide Whether to Fan Out

Cold fan-out is right only if all three hold:

1. **Separable** — the parts do not need each other's intermediate results.
2. **Reducible** — you need each part's _conclusion_, not its full working. The
   parent receives only the final summary; everything the agent read stays in the
   agent's context and never reaches yours. That is the point.
3. **Wide** — doing it inline would flood context with material you would discard
   anyway (many files scanned, many pages read, many jobs inspected).

All three, and at least two genuinely independent branches: fan out. Any one no:
pick another row from the table above.

**Fan out for:** surveying a large surface with unknown structure; independent
research threads (N libraries, N sources, N candidate approaches); analyzing
several failing CI jobs whose failures are unrelated; reviewing files whose
findings do not interact.

**Do not fan out for:**

- Sequential dependencies. If B needs A's output, that is a pipeline. Run A, then
  decide. Chaining means feeding A's finished result into B's prompt, not
  concurrency.
- Work that needs iteration or back-and-forth mid-flight. An agent cannot ask you
  a clarifying question once it is running.
- The judgment itself — a design call, a tradeoff, a question of taste. Workers
  gather; the orchestrator decides. That does not survive delegation.

## Partition the Work

Seams between agent scopes are invisible to every agent: each reports success on
its own slice while the gap between them goes unexamined. Overlaps are cheaper
but waste tokens and produce two half-answers to merge.

Partition along boundaries that already exist — by directory or package, by CI
job, by source, by question. State each boundary both positively and negatively:
what to cover, and what another agent is handling. Where agents may _write_,
give each one a disjoint set of files; two agents editing the same file overwrite
each other.

## Write the Worker Prompt

A cold agent loads CLAUDE.md and a git status snapshot but inherits no
conversation history, and cannot ask a follow-up. Everything else it needs is in
the prompt or it guesses. Under-specified prompts are the largest single cause of
useless fan-out results.

Include:

- **Context it cannot infer** — absolute paths, the branch, what is being built,
  what has already been decided and is not up for revisiting.
- **Scope fence** — what to examine, and explicitly what to leave alone.
- **Output contract** — the shape of the report. Ask for conclusions, plus
  verbatim quotes of anything you intend to copy. Do not ask for raw dumps you
  will have to re-read; that forfeits the context saving.
- **Authority** — read-only or may-edit.
- **An uncertainty rule** — "if you are inferring rather than confirming, say so
  explicitly." Without it, agents report guesses in the same confident register as
  verified facts, and the guess arrives looking like a finding.

**Enforce read-only with tool scoping, not prose.** "Report findings, do not fix
anything" states intent; it does not constrain the agent. Pair it with an agent
type or tool allowlist that actually lacks write access — `Explore` denies
Write/Edit outright. Permission is checked per tool call regardless of what the
prompt says.

## Choose the Agent Type

- **Explore** — read-only search and analysis. Cheapest; skips loading CLAUDE.md
  and git status to stay fast. The default for survey and discovery work.
- **general-purpose** — when the task must write, edit, or run commands, not just
  look.
- **Plan** — research inside plan mode specifically.

## Model Policy

**Standing preference for this repo: run workers on `sonnet`; the orchestrator
stays on the session model and makes every call.** Bounded gather-and-verify work
is what sonnet is good at and cheap for, and synthesis needs the full
conversation context that no worker has.

Note this is a deliberate house rule, not a documented Claude Code best practice.
The documented rationale for the `model` parameter is cost control — routing
high-volume, simple work to cheaper models. Do not present the tiering as
official guidance.

Launch workers in the background so the user can still interject and other work
continues. Use foreground only when the very next action depends on that single
result and nothing else could usefully happen meanwhile. Default concurrency caps
at 20 agents; nesting at 3 deep.

## Fan In

Receiving reports is not synthesis. Before acting on what comes back:

- **Reconcile conflicts explicitly.** Two agents disagreeing means at least one is
  wrong. Resolve it or verify it yourself — never average them, and never quietly
  prefer the more confident one.
- **Spot-check load-bearing claims.** Anything a recommendation rests on gets
  checked against the source. Agents are fluent about things they did not verify.
- **Discount manufactured findings.** An agent asked to find problems will
  usually report some, even when the work is sound. Scope review prompts to
  correctness and stated requirements, and treat the rest as optional.
- **Look for what nobody covered.** Re-read the partition and find the seams.
- **Own the conclusion.** The decision is yours and is stated as yours, not as
  "the agents concluded". You are accountable for their errors.
- **Relay what matters.** Agent reports are never shown to the user. If a finding
  changes the outcome, say it in your own words — do not refer to a report the
  user cannot see, and never paste an agent ID.

## Failure Modes

| Symptom                           | Cause                                   | Fix                                                         |
| --------------------------------- | --------------------------------------- | ----------------------------------------------------------- |
| Two agents did the same work      | Partition stated only positively        | Name what each agent is _not_ covering                      |
| Reports contradict                | Real ambiguity, or one hallucinated     | Verify against source; do not split the difference          |
| Confident but wrong findings      | No uncertainty rule in the prompt       | Require "say so if inferring"; spot-check                   |
| Agent edited what it shouldn't    | Read-only asked for in prose only       | Use `Explore` or a tool allowlist                           |
| Context blown by the results      | Asked for dumps, not conclusions        | Specify an output contract                                  |
| Slower than inline                | Work was not parallel-shaped            | Re-run the three-part test                                  |
| Investigation converged too early | Anchoring on the first plausible theory | Fan out competing hypotheses; have them disprove each other |
| A seam went unexamined            | Scopes did not tile the surface         | Check coverage at fan-in                                    |

## Quick Template

```
Task: <the one question this agent answers>
Context: <paths, branch, what we are building, what is already decided>
Scope: examine <X>. Do NOT touch <Y> — another agent covers it.
Authority: read-only. Report findings; change nothing.
Report back: <exact shape — sections, verbatim quotes where I will copy them>
If you are inferring rather than confirming something, say so explicitly.
```

## Related Skills

- `$authoring-skills` - skill structure and the `context: fork` / `agent:` fields
- `$pr-status-triage` - CI triage, a natural fan-out per failing job
