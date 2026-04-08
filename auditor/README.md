# Bouncy Feet Auditor

QA for BF

The auditor is a separate application, keeping track of Bouncy Feet quality and performance.

Run it:

```bash
cd auditor
npm install
npx playwright install chromium
# run against local env
npm run audit
# run against dev.bouncy-feet.ch
# ENV=staging npm run audit
```
