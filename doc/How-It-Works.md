# Koibumi Workflow

Koibumi is divided into three parts:

- [koibumi](../crates/koibumi) (the main engine)
- [script-engine](../crates/script-engine) (the script engine)
- [koibumi-cli](../crates/koibumi-cli) (an easy-to-use cli for rapid development)

## Koibumi - where the magic begins

Like many other game engine, koibumi takes some steps to setup a game.

### 0 - Basic Requirement

As the first load, koibumi will collect some info about the system(audio, graphic, etc.)

Koibumi could automatically find the default config for itself, but you can change it still.

As the config collected, koibumi will run the initial script(which defined by `init.ks` in default).
