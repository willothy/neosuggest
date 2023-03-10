# Neosuggest

## WIP Suggestion engine for zsh-autosuggest

And I mean VERY WIP, don't use this yet :)

### Features:

- [x] Path fuzzy matching
- [x] Zoxide support (WIP)
- [x] Respects .gitignore and doesn't suggest `.git/`
- [ ] Command & arg completions
    - [ ] Generate completions from clap Command using custom `clap_complete` generator
    - [ ] Completion plugins using dylib loading
- [ ] History integration

### Usage:

In your `.zshrc`:
```zsh
_zsh_autosuggest_strategy_neosuggest() {
    suggestion=$(neosuggest "$BUFFER")        
}

export ZSH_AUTOSUGGEST_STRATEGY=(neosuggest)
```
