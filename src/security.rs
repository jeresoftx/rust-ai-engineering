//! Señales locales para revisar entradas no confiables.

/// Una señal explicable; no equivale a probar intención o seguridad.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrustSignal {
    OverrideAttempt,
    SystemDisclosureRequest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionInspection {
    pub signals: Vec<TrustSignal>,
    pub requires_review: bool,
}

/// Busca patrones mínimos de riesgo y solicita revisión cuando aparecen.
///
/// Una lista de patrones no detecta todos los ataques ni prueba que una entrada
/// inocente sea segura. Su función es hacer visible una frontera de confianza.
pub fn inspect_instruction(input: &str) -> InstructionInspection {
    let normalized = input.to_lowercase();
    let mut signals = Vec::new();
    if normalized.contains("ignora las instrucciones anteriores")
        || normalized.contains("ignore previous instructions")
    {
        signals.push(TrustSignal::OverrideAttempt);
    }
    if normalized.contains("revela el sistema") || normalized.contains("reveal the system") {
        signals.push(TrustSignal::SystemDisclosureRequest);
    }
    let requires_review = !signals.is_empty();
    InstructionInspection {
        signals,
        requires_review,
    }
}
