# Keycloak themes

Keycloak theme development for Bouncy Feet.

Docs: https://www.keycloak.org/ui-customization/themes

## Setup

```bash
cd keycloak_theme
sudo docker compose up --build
```

Then open http://localhost:8080 and log in with "admin" and "unsafe-dev-password".

In realm settings, set the login theme to "bouncy-feet".

Now start editing files in ./themes/login