# Why

There are several similar tools to this one. Depending on your use case you
might want to check them out instead of this crate

- [git-repo-updater](https://github.com/earwig/git-repo-updater)
- [huber](https://github.com/innobead/huber)
- [nix](https://nixos.wiki/wiki/Nix_package_manager)

However, I have published this crate. My reasons are:

1. I wanted to easily download, build and install git projects.
2. I wanted to know what I had installed with the project manager.
3. I wanted to update to the latest changes available without having to
manually download the new version and build it.
4. I wanted to edit the project setup
5. I wanted to be able to always go back to the previous version if things
stopped working.
6. I wanted to uninstall the software once I was done with it.
7. I wanted to install any prpject at any level of development, so it couldn't
be a managed package manager and it couldn't rely on published releases.

This requirements came from using the [Helix editor](https://helix-editor.com/)
❤️❤️❤️ before it was as mature as it is today. I still want to follow the
development closer than the releases that they offer, so I created this
(hopefully useful) tool.

All this being said, I personally think that unless you want to try the latest
commit (and you probably don't), you are better off using huber or nix

# What does it do?

- The things stated in the why section
- Provide some build suggestions
- Some ability to fix your errors and edit the configuration for the projects

# What does it not do

- Automatically know how to build a random project.
- Download exactly the dependencies you need to build the project.
- Ensure that what could be built yesterday can be built today.
- Make a good general purpose project manager


# Known bugs

- You can only install via `https` clones and not `ssh`. Given that you
shouldn't really be interacting with the repos this shouldn't be a mayor
problem, but it's a bummer


# Architecture

This project is kind of two different things rolled into one. On the one hand
there is the `amisgitpm` and on the other one, every other workspace,
for convenience we will call it `agpm`.

`amisgitpm` is a collection of traits, that when implemented and combined can
potentially result in a working ""project manager"".

`agpm_pm`, `agpm_store`, `agpm_dirs`, `agpm_project` are generic implementations
of the traits defined in `amisgitpm`. `agpm_pm` defines an extra trait to
`Interactions`, to separate the nitty-gritty implementation detail of the
interactions from the implementation of the `PMInteractive` trait. That's where
`agpm_interactions` and `agpm_suggestions` come into play. They offer an
implementation with some extra optional features in `agpm_suggestions`.

`agpm` is a crate that glues all the parts together so that they are one
concrete implementation that uses all the parts I just described. It makes the
concrete types used available through its `lib.rs file`.

# Development

Currently, the testing is linux dependent

# Install

```bash
cargo b
cargo r -- bootstrap
```
