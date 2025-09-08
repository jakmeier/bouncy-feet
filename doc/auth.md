# User authentication and authorization principles

BouncyFeet follows the principle that all resources should be as open as possible without giving up user privacy or security.
Content shared as public to all BouncyFeet users should also be available to unregistered, anonymous users.

When authorization is required in a service, this service should only have the minimum amount of necessary data about the user.
Often, this is just a unique user identifier plus a authorization assertion.


## For what a login is not required

Public resources on BouncyFeet are accessible without a login, hence there is no need for user authentication.

User data, such as course progression, is primarily stored in local storage of the user's device. Again, no authentication is necessary.


## For what a login is required

Uploading data to BouncyFeet servers for persistent storage always requires user authentication and in most cases authorization.
Examples of data include dance videos, user progression backup, and opt-in usage statistics.


# Technical AUTH implementation

BouncyFeet uses a central Identity Provider (IdP) on https://auth.bouncy-feet.ch.
It runs a [Keycloak](https://www.keycloak.org/) instance.
Each BouncyFeet service has a registered client, preferably using the OpenID Connect protocol.

Below is a list of services and how they use the IdP.


## BouncyFeet main app frontend <-> API server

Login:
The frontend communicates with the API server to complete an [Authorization Code Flow](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1).
The access token remains on the backend server, never to be revealed to the user.
Cookie-based sessions are used to authenticate the user.

Authorization:
The backend checks user permissions, currently only based on the user id.

Keycloak configuration:
Only the standard flow enabled.
Client must be authenticated (using client_id + client_secret).

![Keycloak client configuration screenshot matching the description in text](./img/auth/api-backend-client.png)


## BouncyFeet JWT in the frontend

**Discussion**: It would be possible to retrieve a JWT right in the frontend, by using, for example, keycloak.js and using a public client on the Keycloak side. The benefit: No backend session management required to access PeerTube (and other services). The backend Bouncy Feet API server could potentially be decoupled from the video hosting platform.

However, a user will be logged in to Keycloak on the backend service anyway. Maintaining two different user sessions is hard to hide from the use experience and makes login flow more complex to implement. It is much easier to keep the Keycloak token on the backend only. This allows to never expose a JWT to the user.

But the user must still be able to upload videos directly from the frontend, without going through the backend. This happens through a separate API access token, which unfortunately adds complexity. But that is the minimal amount of added complexity necessary to avoid the overhead through the backend. See [BouncyFeet's PeerTube API access](#bouncyfeets-peertube-api-access) for more.

**Current Design**: The frontend never sees the JWT. It stays on the backend service.

## BouncyFeet's PeerTube instance

This is the default PeerTube frontend.
See [BouncyFeet's PeerTube API access](#bouncyfeets-peertube-api-access) for direct access from the BouncyFeet frontend.

Login:
Vanilla [PeerTube](https://github.com/Chocobozzz/PeerTube) handles user management internally, with a bundled OAuth 2.0 server for API access.
BouncyFeet's instance disables the default user type and only allows users created through the [OpenID Connect plugin](https://framagit.org/framasoft/peertube/official-plugins/tree/master/peertube-plugin-auth-openid-connect).

Authorization:
Permissions in PeerTube.

Keycloak configuration:
Only the standard flow enabled.
Client must be authenticated (using client_id + client_secret).

![Keycloak client configuration screenshot matching the description in text](./img/auth/api-backend-client.png)

PeerTube configuration:

- Requires the [OpenID Connect plugin](https://framagit.org/framasoft/peertube/official-plugins/tree/master/peertube-plugin-auth-openid-connect)
- Discover URL must be configured to `https://auth.bouncy-feet.ch/realms/bouncyfeet/.well-known/openid-configuration`
- Client ID and Client secret must be copied over from Keycloak


## BouncyFeet's PeerTube API access

This is for direct access from JS code in the BouncyFeet app to the PeerTube API, for example to upload a video.

Login:
The frontend makes a call to the backend API route `/peertube/token`. The backend uses a JWT access token from the already-logged-in user and sends it to the token-exchange endpoint from the OpendID connect plugin on the PeerTube instance. This yields a token usable on PeerTube's API.

Authorization:
Keycloak puts the PeerTube token exchange endpoint in the JWT audience.
Final API accesses are authorized by permissions in PeerTube.

Keycloak configuration:
Configure the exact token exchange URL (e.g. https://tube.bouncy-feet.ch/plugins/auth-openid-connect/router/token-exchange) to be included in the audience. We use a client scope with a audience mapper for this. The mapper must be in a client scope that is added as "default" type to the client for the PWA frontend.

![Keycloak audience mapper configuration screenshot matching the description in text](./img/auth/aud-mapper.png)

![Keycloak client scope screenshot matching the description in text](./img/auth/add_client_scope.png)

PeerTube configuration:
No additional config other than described in [BouncyFeet's PeerTube instance](#bouncyfeets-peertube-instance).
