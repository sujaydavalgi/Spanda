# Publishing `@spanda/web`

The Control Center React panel ships from `packages/web` as **`@spanda/web`**.

## Versioning

Follow semver aligned with the Spanda release (`package.json` → `version`). Bump **minor** for additive UI/API client changes; **major** for breaking prop or export changes.

## Local dry-run

```bash
cd packages/web
npm run build
npm pack
```

## CI publish scaffold

`.github/workflows/publish-npm-web.yml` runs `npm publish --dry-run` on every PR and publishes on tags:

```text
npm-web-v0.4.0
```

## Before first public publish

1. Set `"private": false` in `package.json`.
2. Add `publishConfig.access: "public"`.
3. Configure `NPM_TOKEN` repository secret.
4. Tag `npm-web-vX.Y.Z`.

## Scope

This package contains the playground IDE and `ControlCenterPanel`. It does **not** include the Tauri desktop shell (`@spanda/control-center-desktop`).
