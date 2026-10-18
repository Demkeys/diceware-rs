## diceware-rs

Simple passphrase generator that uses the diceware method. The wordlist has been taken from the EFF website and has been embedded into the binary, so the program is self contained and can be placed in any directory to be run.

![](./diceware-screenshot.png)

### Usage
- Install via cargo:
```
cargo install --git https://github.com/Demkeys/diceware-rs
```
- Run 'diceware-rs'. Program will generate a passphrase, print it out and exit.

---
### TODO
- [ ] More customization options using clap to provide functionality for flags and agrs.
  - [ ] Case (--case)
    - [ ] camelCase
    - [ ] kebab-case
    - [ ] snake_case
    - [ ] PascalCase
    - [ ] SCREAMING_SNAKE_CASE
  - [ ] Include numbers (--numbers)
    - [ ] Attached to one of the words (attached)
    - [ ] Separate from all the words (separate)
- [ ] Allow persistent custom settings via TOML config file.
- [x] Set up a way to install binary via cargo install.
  - Turns out this was already possible since this is a binary crate. Specifically using
  ```
  cargo install --git <repo link>
  ```



