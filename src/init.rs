use anyhow::Result;

pub fn init() -> Result<()> {
    println!(
        r#"
_zsh_autosuggest_strategy_neosuggest() {{
	suggestion=$(neosuggest run "$BUFFER")
}}

neosuggest-accept() {{
	zle autosuggest-accept
	zle autosuggest-fetch
}}

zle -N neosuggest-accept 

ZSH_AUTOSUGGEST_IGNORE_WIDGETS+=neosuggest-accept
			 "#
    );
    Ok(())
}
