use rust_ai_engineering::retrieval::{
    Embedding, SimilarityError, cosine_similarity, rank_by_similarity,
};

#[test]
fn cosine_similarity_is_one_for_identical_vectors() {
    let left = Embedding::new(vec![1.0, 2.0]).expect("vector válido");
    let right = Embedding::new(vec![1.0, 2.0]).expect("vector válido");

    assert_eq!(cosine_similarity(&left, &right), Ok(1.0));
}

#[test]
fn cosine_similarity_rejects_incompatible_dimensions() {
    let short = Embedding::new(vec![1.0]).expect("vector válido");
    let long = Embedding::new(vec![1.0, 2.0]).expect("vector válido");

    assert_eq!(
        cosine_similarity(&short, &long),
        Err(SimilarityError::DimensionMismatch)
    );
}

#[test]
fn ranking_uses_identifier_as_a_stable_tie_breaker() {
    let query = Embedding::new(vec![1.0, 0.0]).expect("vector válido");
    let first = Embedding::new(vec![1.0, 0.0]).expect("vector válido");
    let second = Embedding::new(vec![1.0, 0.0]).expect("vector válido");

    let ranking =
        rank_by_similarity(&query, [("zeta", &first), ("alfa", &second)]).expect("ranking válido");

    assert_eq!(ranking[0].id, "alfa");
    assert_eq!(ranking[1].id, "zeta");
}
