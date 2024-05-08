# Usage

## Installation & Configuration for Client/Server

Refer to [configuration.md](configuration.md) for sample configuration files and documentation of the available options.
As an example I'll use masterserver publicly hosted by me.

### Client

1. Download `masterservers.vdf`.
2. Copy `masterservers.vdf` to the `platform/config` folder and replace the file.
3. Make `masterservers.vdf` read-only.

### Server

1. Go to the `cfg/server.cfg`.
2. Open and at the bottom, write `setmaster add phantasma.ikanaide.pw:27010`.
3. Restart game server.
