//! Representaciones vectoriales y recuperación local reproducible.

/// Un vector local validado para los ejemplos del curso.
#[derive(Clone, Debug, PartialEq)]
pub struct Embedding {
    values: Vec<f32>,
}

impl Embedding {
    /// Construye un vector no vacío con componentes finitos.
    pub fn new(values: Vec<f32>) -> Result<Self, EmbeddingError> {
        if values.is_empty() {
            return Err(EmbeddingError::Empty);
        }
        if values.iter().any(|value| !value.is_finite()) {
            return Err(EmbeddingError::NonFinite);
        }
        Ok(Self { values })
    }
}

/// Errores de construcción de una representación local.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmbeddingError {
    Empty,
    NonFinite,
}

/// Errores al comparar representaciones.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimilarityError {
    DimensionMismatch,
    ZeroMagnitude,
}

/// Un resultado de recuperación que conserva identidad y puntuación.
#[derive(Clone, Debug, PartialEq)]
pub struct RankedDocument {
    pub id: String,
    pub score: f32,
}

/// Un documento local con la representación que usa el índice.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexedDocument {
    pub id: String,
    pub text: String,
    embedding: Embedding,
}

impl IndexedDocument {
    pub fn new(id: impl Into<String>, text: impl Into<String>, embedding: Embedding) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            embedding,
        }
    }
}

/// Un resultado recuperado con su texto y procedencia local.
#[derive(Clone, Debug, PartialEq)]
pub struct RetrievedDocument {
    pub id: String,
    pub text: String,
    pub score: f32,
}

/// Errores de contrato del índice local.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexError {
    DimensionMismatch,
    DuplicateId,
    InvalidQuery,
}

/// Un índice didáctico en memoria, sin persistencia ni acceso a red.
#[derive(Clone, Debug, Default)]
pub struct InMemoryIndex {
    dimension: usize,
    documents: Vec<IndexedDocument>,
}

impl InMemoryIndex {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            documents: Vec::new(),
        }
    }

    /// Inserta un documento solo si respeta la dimensión y el identificador único.
    pub fn insert(&mut self, document: IndexedDocument) -> Result<(), IndexError> {
        if document.embedding.values.len() != self.dimension {
            return Err(IndexError::DimensionMismatch);
        }
        if self
            .documents
            .iter()
            .any(|current| current.id == document.id)
        {
            return Err(IndexError::DuplicateId);
        }
        self.documents.push(document);
        Ok(())
    }

    /// Recupera hasta `limit` documentos ordenados y conserva su procedencia.
    pub fn search(
        &self,
        query: &Embedding,
        limit: usize,
    ) -> Result<Vec<RetrievedDocument>, IndexError> {
        if query.values.len() != self.dimension {
            return Err(IndexError::DimensionMismatch);
        }

        let mut results = self
            .documents
            .iter()
            .map(|document| {
                cosine_similarity(query, &document.embedding)
                    .map(|score| RetrievedDocument {
                        id: document.id.clone(),
                        text: document.text.clone(),
                        score,
                    })
                    .map_err(|_| IndexError::InvalidQuery)
            })
            .collect::<Result<Vec<_>, _>>()?;
        results.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| left.id.cmp(&right.id))
        });
        results.truncate(limit);
        Ok(results)
    }
}

/// Calcula similitud coseno para dos vectores de igual dimensión.
pub fn cosine_similarity(left: &Embedding, right: &Embedding) -> Result<f32, SimilarityError> {
    if left.values.len() != right.values.len() {
        return Err(SimilarityError::DimensionMismatch);
    }

    let dot = left
        .values
        .iter()
        .zip(&right.values)
        .map(|(a, b)| a * b)
        .sum::<f32>();
    let left_norm = left.values.iter().map(|value| value * value).sum::<f32>();
    let right_norm = right.values.iter().map(|value| value * value).sum::<f32>();
    let denominator = left_norm.sqrt() * right_norm.sqrt();

    if denominator == 0.0 {
        return Err(SimilarityError::ZeroMagnitude);
    }

    Ok(dot / denominator)
}

/// Ordena candidatos por similitud descendente y por identificador en empates.
pub fn rank_by_similarity<'a>(
    query: &Embedding,
    candidates: impl IntoIterator<Item = (&'a str, &'a Embedding)>,
) -> Result<Vec<RankedDocument>, SimilarityError> {
    let mut ranked = candidates
        .into_iter()
        .map(|(id, embedding)| {
            cosine_similarity(query, embedding).map(|score| RankedDocument {
                id: id.to_owned(),
                score,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    ranked.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(ranked)
}
