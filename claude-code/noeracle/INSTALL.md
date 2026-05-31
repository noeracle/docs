# Install the noeracle Claude Code skill

For Stellar Istanbul hackathon teams using Claude Code.

## What this does

Drops a noeracle skill into your Claude Code install so that any time you tell Claude "build me X on Stellar that needs BTC price" (or similar), it knows the entire integration pattern — Pattern A vs B, the asset-tag table, tool-version pins, deploy walkthrough, and common errors. No more 30-minute detours through the docs.

## One-time install

```bash
# Pick one — both work.

# Option A: clone the noeracle docs repo (has the latest skill).
git clone https://github.com/noeracle/docs ~/noeracle-docs
mkdir -p ~/.claude/skills
cp -r ~/noeracle-docs/claude-code/noeracle ~/.claude/skills/

# Option B: copy from anywhere you already have the skill folder.
mkdir -p ~/.claude/skills
cp -r /path/to/skill/noeracle ~/.claude/skills/
```

Verify:

```bash
ls ~/.claude/skills/noeracle/
# SKILL.md  REFERENCE.md  templates/  INSTALL.md
```

## Use it

Open Claude Code in any directory:

```bash
claude
> I want to build a perpetual DEX MVP on Stellar that uses BTC/USD prices from noeracle.
```

Claude will detect the noeracle skill is relevant, walk you through the version checks, scaffold a Pattern B Soroban contract + TS demo, deploy to testnet, and run an end-to-end test.

You can also invoke it explicitly:

```bash
> /noeracle scaffold a prediction market
```

## What's inside

| File | Purpose |
| --- | --- |
| `SKILL.md` | The main playbook Claude reads — patterns, gotchas, recipe |
| `REFERENCE.md` | Deep reference (architecture diagram, contract spec, threat model) |
| `templates/consumer/` | Cargo.toml + lib.rs + test.rs — Pattern B template |
| `templates/demo/` | package.json + demo.mjs — TS demo template |
| `templates/deploy.sh` | One-shot build + deploy + init |

## Updating

The skill is just markdown + templates. When noeracle updates (new feeds, new contract address, new SDK version), pull the latest:

```bash
cd ~/.claude/skills/noeracle
git pull   # if installed via git
```

or re-copy from the noeracle docs repo.
