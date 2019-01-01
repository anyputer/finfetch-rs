# finfetch-rs
A port of [arkizenty's finfetch](https://github.com/arkizenty/finfetch) written in Rust. This is incomplete (and isn't without any issues), but mainly serves as a hint on writing better code that does not look like a comet hitting ASCII art of a whale.

### What I've not implemented:

- Colors

### What I changed / fixed:

- The clear function which results in this: <img src="https://cdn.discordapp.com/attachments/422293824770146306/529086219859263500/unknown.png" alt="spam because the clear command doesn't work"> on Windows.
- Oh and did I mention that the [console](https://github.com/mitsuhiko/console) crate I'm using supports terminal color cross-platform, so it doesn't end up as a mess of ANSI escape sequences that don't work because the [aurora](https://github.com/logrusorgru/aurora) Golang package is made to work with fancy Linux terminals only?
- Timezone names are different

**This README isn't meant to be taken personally.**

[arkizenty's GitHub account](https://github.com/arkizenty)
