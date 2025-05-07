## Install 

Install binary
```bash
cargo install --git https://github.com/vuvoth/sui-env-report --locked
```
Add this script to ~/.zshrc

```bash
precmd() {
    sui-env-report
}
```
