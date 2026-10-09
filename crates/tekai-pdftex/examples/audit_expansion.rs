//! Bounded diagnostics for the experimental expansion engine.
//! Does not read project files, load formats, or write artifacts.

use std::error::Error;
use std::time::Instant;

use tekai_pdftex::{CatCode, ExpansionEngine, MacroDefinition, Token};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("scopes") if args.len() == 5 => {
            let definitions = args[1].parse::<usize>()?;
            let depth = args[2].parse::<usize>()?;
            let replacement_tokens = args[3].parse::<usize>()?;
            let mutating = match args[4].as_str() {
                "read-only" => false,
                "local" => true,
                _ => return Err("scope mode must be read-only or local".into()),
            };
            if definitions > 2000 || depth > 64 || replacement_tokens > 128 {
                return Err("fixture exceeds the bounded diagnostic budget".into());
            }
            let token_budget = definitions
                .checked_mul(replacement_tokens)
                .and_then(|tokens| tokens.checked_mul(depth + 1));
            if token_budget.is_none_or(|tokens| tokens > 4_000_000) {
                return Err("fixture exceeds the bounded diagnostic budget".into());
            }
            let mut source = String::new();
            for _ in 0..depth {
                source.push('{');
                if mutating {
                    source.push_str(r"\def\auditlocal{x}");
                }
            }
            source.push('x');
            source.push_str(&"}".repeat(depth));
            let mut engine = ExpansionEngine::new(&source);
            for index in 0..definitions {
                engine.define_macro(
                    format!("audit{index}"),
                    MacroDefinition::new(0, vec![Token::Character {
                        ch: 'x', catcode: CatCode::Letter,
                    }; replacement_tokens]),
                );
            }
            let started = Instant::now();
            let output = engine.expand_all()?;
            let elapsed = started.elapsed();
            println!(
                "definitions={definitions} depth={depth} replacement_tokens={replacement_tokens} \
                 mutating={mutating} token_bytes={} output_tokens={} expansion_ms={:.3}",
                std::mem::size_of::<Token>(), output.len(), elapsed.as_secs_f64() * 1000.0,
            );
        }
        Some(mode @ ("division-overflow" | "division-overflow-edef" | "hex-unicode"))
            if args.len() == 1 =>
        {
            // Finite inputs must return an expansion error rather than panic.
            let source = match mode {
                "division-overflow" => concat!(
                    r"\count0=-9223372036854775807 \advance\count0 by -1 ",
                    r"\number\numexpr\count0/-1\relax",
                ),
                "division-overflow-edef" => concat!(
                    r"\count0=-9223372036854775807 \advance\count0 by -1 ",
                    r"\edef\auditresult{\number\numexpr\count0/-1\relax}",
                ),
                _ => r"\pdfunescapehex{é}",
            };
            let mut engine = ExpansionEngine::new(source);
            match engine.expand_all() {
                Ok(output) => println!("output={output:?}"),
                Err(error) => println!("expansion_error={error}"),
            }
        }
        _ => return Err("usage: audit_expansion scopes DEFINITIONS DEPTH TOKENS read-only|local, or division-overflow|division-overflow-edef|hex-unicode".into()),
    }
    Ok(())
}
