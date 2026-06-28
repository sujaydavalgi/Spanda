# @davalgi-spanda/sdk

Official TypeScript SDK for Spanda Control Center API v1.

## Install

```bash
npm install @davalgi-spanda/sdk
```

From this monorepo:

```bash
npm ci --prefix sdk/typescript && npm run build --prefix sdk/typescript
```

## Quick start

```typescript
import { SpandaClient } from "@davalgi-spanda/sdk";

const client = SpandaClient.local();
const report = await client.readiness("rover.sd");
console.log(report.score);
```

## Documentation

- [docs/sdk-typescript.md](../../docs/sdk-typescript.md)
- [docs/control-center-api.md](../../docs/control-center-api.md)

## Publish

Requires GitHub secret `NPM_TOKEN` (granular token with `@davalgi-spanda/*` write + Bypass 2FA). Tag `npm-sdk-v*` triggers [.github/workflows/publish-sdk-typescript.yml](../../.github/workflows/publish-sdk-typescript.yml). npm write tokens expire within 90 days — rotate `NPM_TOKEN` before expiry.
