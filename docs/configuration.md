# Configuration

Configure Phantasma by passing commands into CLI or editing its [TOML](https://toml.io/en/) configuration file, `cfg.toml`.

A config file can be specified with `phantasma --config /path/to/cfg.toml`.

You MUST set the following options:

- `ip`: the ip where your instance is hosted.
- `port`: the port of your instance.

Other available options:

- `challenge`: Time in seconds while challenge is valid.
- `server`: Time in seconds while server is valid.
- `level`: Set logging level with possible values: 0-5, off, error, warn, info, debug, trace.