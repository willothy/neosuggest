# Neosuggest

## WIP Suggestion engine for zsh-autosuggest

### Features:

- [x] Path fuzzy matching
- [x] Zoxide support (WIP)
- [ ] Command & arg completions
- [ ] History integration

### Usage:

In your `.zshrc`:
```zsh
_zsh_autosuggest_strategy_neosuggest() {
    suggestion=$(neosuggest "$BUFFER")        
}

export ZSH_AUTOSUGGEST_STRATEGY=(neosuggest)
```
