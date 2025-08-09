## BouncyFeet main app / PWA

PROD: https://app.bouncy-feet.ch/
DEV: https://dev.bouncy-feet.ch/

This contains all the UI code.

Architecture decisions:

- Built with [SvelteKit 2](https://sveltekit.io/blog/sveltekit-2)
- Hosted with [bun](https://bun.com/) using [@sveltejs/adapter-node](https://www.npmjs.com/package/@sveltejs/adapter-node) to build
- No database
- Not a static website (server-side rendering and dynamic routes are too useful)


## BouncyFeet API server

PROD: https://stats.bouncy-feet.ch/
DEV: https://dev-api.bouncy-feet.ch/

This is where application data of users are stored.

Architecture decisions:

- Built with [axum](https://github.com/tokio-rs/axum)
- [PostgreSQL](https://www.postgresql.org/) database
- Decoupled from video hosting
- An anonymous user could access all public content without ever connecting to this server
- Ideally, most functionalities in the app work without active connection to the API server. While offline, changes can be buffered and synced on the next connection to the API server.


## Video hosting and moderation service

PROD: https://tube.bouncy-feet.ch/
DEV: https://dev-tube.bouncy-feet.ch/

Architecture decisions:

- Out-of-the-box [PeerTube](https://github.com/Chocobozzz/PeerTube) with custom themes and plugins
- Decoupled from the API server
- To show videos in the BouncyFeet app, they are embedded directly from the PeerTube instance
- BouncyFeet manages videos through PeerTube0s REST API
- Uploading videos puts them in a channel of the user
- Users have access to the standard PeerTube frontend and can use it to manage permissions on videos or entirely delete them from the platform
- Moderators also use the standard PeerTube frontend


## Identity Provider

PROD: https://auth.bouncy-feet.ch/realms/bouncyfeet
DEV: https://auth.bouncy-feet.ch/realms/bouncyfeet-dev

Architecture decisions:

- Out-of-the-box [Keycloak](https://www.keycloak.org/) with custom themes and plugins