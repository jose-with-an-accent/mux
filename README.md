# Mux.rs
This project is made as:
- A learning experience, to learn about Rust and be able to keep up with its borrowing system
- A tool to create static pages and client content with less effort.
- An experiment to build an SSG tool and to understand how these work.

This project is not meant to be used by the general public, and is just meant to simplify creating simple websites.

Config can be done through a wizard launched with ``mux-rs new __`` or by modifying template TOML files.
## Project Goals
[ ] An easy way to start building websites from a template

[ ] Effortless translation support, particularly with English

[ ] A simple and consistent way to add navigation, albeit with customization possible

[ ] (future) being able to fetch data and update the page from that.

Translation and variable support will use data tags, as such: ``data-tr`` which can be mapped to a language file in the ``i18n`` folder.
