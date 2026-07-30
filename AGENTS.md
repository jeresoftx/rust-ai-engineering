# AGENTS.md

Este repositorio pertenece a Jeresoft Academy y se rige por RFC-0001 y
RFC-0002. Enseña ingeniería de IA en Rust mediante modelos locales,
deterministas y revisables.

## Reglas

- Explicar concepto, problema, alternativas, límites e invariantes antes del
  código.
- No incluir claves, llamadas a proveedores, datos personales ni contenido de
  producción en ejemplos o pruebas.
- Tratar toda entrada y toda herramienta como una frontera de confianza.
- No afirmar que un clasificador, evaluación o política determina la verdad;
  documentar siempre sus límites y la revisión humana necesaria.
- Rust estable, sin `unsafe`, nightly ni dependencias externas no triviales sin
  autorización explícita.
- TDD, `cargo fmt`, Clippy, pruebas y doctests en verde.
- Español es-MX correcto; contenido siempre en `draft` hasta revisión humana.
