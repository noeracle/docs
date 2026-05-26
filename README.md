# noeracle/docs

Source for **[docs.noeracle.org](https://docs.noeracle.org)** — the user-facing
documentation for [Noeracle](https://noeracle.org), the pull-based price oracle
for Stellar.

## Layout

| Path | What |
|---|---|
| `docs/` | The canonical MDX content. Source of truth for every page. |
| `docs_website/` | Docusaurus 3.10 build harness. Reads from `../docs`. |

## Pages

- `docs/intro.mdx` — landing
- `docs/get-started/quickstart.mdx`
- `docs/integration/patterns.mdx`
- `docs/concepts/architecture.mdx`
- `docs/concepts/threat-model.mdx`
- `docs/reference/sdk.mdx`
- `docs/reference/contract.mdx`
- `docs/project/roadmap.mdx`

## Develop

```bash
cd docs_website
npm install
npm start        # http://localhost:3000  — hot reload
npm run build    # static build → docs_website/build/
npm run serve    # serve the production build locally
```

## Deploy

Pushes to `main` auto-deploy via Vercel to **docs.noeracle.org**. The Vercel
project's **Root Directory** is set to `docs_website`.

## Contributing

Each page targets one screen of reading. Keep prose tight, code examples
copy-pasteable, and tables one line per row. Sidebar order is explicit in
`docs_website/sidebars.ts`.

## License

MIT — see [LICENSE](LICENSE).
