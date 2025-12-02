# How distributed state is organized on the Bouncy Feet platform

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

## Client session

A Bouncy Feet client application can be in different states.

```rust
enum ClientState {
   Init,
   Guest(ClientSessionId),
   LoggedIn(ClientSessionId),
}
```

In the `Init` state, the user should decide to log in or continue as guest
before doing anything else.

In the `Guest` state, a user can use the app on a single device. 

To connect multiple devices, a user account on the API server must be created.
Then, one can connect an existing guest sessions to the user account. This will
transition the client into the `LoggedIn` state.

In the `LoggedIn`, the combination of activities and collections across all
devices will then be merged and made available on all devices.

The `ClientSession` is a globally unique identifier, generated on the client.
The API server must check that sessions connected to a user can only be accessed
by the authenticated user. For guest sessions, there are no checks. Therefore,
it is important to keep the client session id private.

## Distributed state guidelines

To enable dynamic merging of existing client sessions, we need a robust plan for
how data is shared. Each client session can also be used in offline mode, which
means out-of-order modification of shared data is always possible.

To avoid major data inconsistency issues, every piece of the user's state follow a few rules:

1. For each data point, one single point of authority serializes all
   modifications to it. It can be one single server or one single client.
2. When modifying remote data, keep the last synchronized state separate from
   the modifications. Synchronize by sending the modifications to the point of
   data authority, get the new state, then delete the locally buffered modifications.
3. Every modification needs an associated timestamp, to resolve conflicts.
4. Modifications should be semantically accurate. Example: "Increase score by
   10" instead of "Set score to 110".
5. Modifications should be as granular as possible. Example: Overwrite field of
   an object rather than overwriting the entire object.

## How the guidelines are applied in the project

### API server as data master

The API sever acts as a central master for creating new user IDs and for
managing general meta data of the user, such as the public name. Client sessions
can modify the local view if they are offline, and sync the change to the API
server later. To resolve conflicts between different modifications, a timestamp
of the modification needs to be included.


### Client data authority

1. Each client has authority over the activities(*) done through it. To this list
   of activities, only this client session can append.
2. The API server keeps a copy of all activity lists and presents a global user
   state. This composition of client states is a data point of its own, which
   the API server has data authority over. Only through that, clients get to see
   each others changes.
3. User meta data can be modified freely on a client. Modifications are stored
   locally until they have been successfully synced with the API server.

(*) Recorded activities are yet to be defined how they look in detail.
