# Distributed state

Bouncy Feet can be used in offline mode, across different devices and sessions.

This document describes how to ensure a consistent state in this setting.

## Architecture overview

There is a single API server and zero to many active client sessions per user.

```txt
                     +------------------+
                     |     API Server   |
                     +------------------+
                              |
        -------------------------------------------
        |                     |                     |
+------------------+  +---------------+  +-----------------+
| Client session 1 |  | Client s. 2   |  | Client s. 3     |
|------------------|  |---------------|  |-----------------|
| Browser session  |  | Android PWA   |  | Expired browser |
| on a desktop     |  |               |  | session         |
+------------------+  +---------------+  +-----------------+
```

Clients perform local modification to the state. If they are online, they will
then synchronize the state with the API server, which will allow other clients
to observe the changes.

There is no limit for how long client can be offline. The API server therefore
cannot assume it knows all the latest changes. It cannot even know how many
clients there are, since a guest session on a user's device has not even been
linked to a user id, yet.


## API server as data master

The API sever acts as a central master for creating new user IDs and for
managing general meta data of the user, such as the public name. Client sessions
can modify the local view if they are offline, and sync the change to the API
server later. To resolve conflicts between different modifications, a timestamp
of the modification needs to be included.


## Client session data authority

Each client session keeps a replica of all the relevant data provided by the API
server. This data is immutable, with a few important exceptions.

1. The activities recorded on this client can be appended to. The list is
   cleared after it has been successfully synced with the API server.
2. User meta data can be modified freely. Modifications are stored locally until
   they have been successfully synced with the API server.

(As more mutable data is added, please add them to the list above.)

Recorded activities are  initially