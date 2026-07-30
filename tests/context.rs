use rust_ai_engineering::context::{ContextBuilder, ContextError, ContextFragment};

#[test]
fn context_keeps_sources_and_respects_the_budget() {
    let builder = ContextBuilder::new(11).expect("presupuesto válido");
    let bundle = builder
        .assemble([
            ContextFragment::new("rfc", "doce letras"),
            ContextFragment::new("extra", "no cabe"),
        ])
        .expect("fragmentos válidos");

    assert_eq!(bundle.text, "doce letras");
    assert_eq!(bundle.sources, vec!["rfc"]);
    assert_eq!(bundle.used_chars, 11);
}

#[test]
fn context_rejects_fragments_without_provenance() {
    let builder = ContextBuilder::new(10).expect("presupuesto válido");

    assert_eq!(
        builder.assemble([ContextFragment::new("", "texto")]),
        Err(ContextError::MissingSource)
    );
}
