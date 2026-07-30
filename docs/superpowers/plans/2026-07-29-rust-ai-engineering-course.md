# Plan de implementación de Rust AI Engineering

**Estado:** draft

## Objetivo

Entregar un curso y crate educativo de ingeniería de IA en Rust que enseñe a
diseñar sistemas con representaciones, recuperación, contexto, herramientas,
agentes, fronteras de confianza y evaluación reproducible. Prioriza evidencia,
límites explícitos y revisión humana sobre demostraciones llamativas.

## Alcance y límites

El curso cubre embeddings como representaciones numéricas, búsqueda vectorial
local, recuperación aumentada por generación (RAG), prompting estructurado,
function calling, herramientas con capacidades limitadas, agentes acotados,
MCP como contrato de integración, seguridad de LLMs y evaluación. El crate
implementa modelos mínimos y deterministas para razonar sobre contratos; no
implementa modelos de lenguaje ni llama servicios externos.

No se usan `unsafe`, nightly, claves, red, proveedores de IA, datos personales
ni dependencias externas sin autorización. Las heurísticas de seguridad y las
métricas se presentan como señales falibles, nunca como decisiones autónomas
definitivas.

## Fases

1. Fundación: alcance, evidencia, glosario inicial y contrato del crate.
2. Recuperación: representaciones, similitud y un índice local.
3. Contexto: RAG, presupuesto y procedencia de fragmentos.
4. Orquestación: prompting, herramientas, agentes y contrato MCP mínimo.
5. Seguridad: prompt injection, capacidades y límites de confianza.
6. Evaluación: casos reproducibles, métricas, límites y cierre editorial.

## Ruta crítica

Fundación → recuperación → contexto → orquestación → seguridad → evaluación.
Cada fase se divide en especificación, modelo probado y capítulo didáctico.

## Seguimiento operativo

El [GitHub Project #25](https://github.com/users/jeresoftx/projects/25) es la
representación operativa del plan. Su vista principal se agrupa por milestone;
cada issue está asignado a `jeresoftx`, tiene prioridad, duración estimada,
fecha de roadmap, dependencia, validación y criterio de cierre.

- [x] #1 Coordinar plan y trazabilidad.
- [x] #2 Documentar alcance, evidencia y límites.
- [x] #3 Implementar similitud y ranking determinista.
- [x] #4 Implementar índice local y recuperación.
- [x] #5 Escribir capítulo de recuperación.
- [x] #6 Implementar ensamblado RAG con procedencia.
- [x] #7 Escribir capítulo de RAG y presupuesto.
- [x] #8 Implementar registro de herramientas limitado.
- [x] #9 Implementar agente acotado y contrato MCP.
- [x] #10 Escribir capítulo de herramientas, agentes y MCP.
- [ ] #11-#12 Completar seguridad y límites de confianza.
- [ ] #13-#15 Completar evaluación y cierre editorial.

## Criterio de cierre

El curso queda completo como `draft` cuando cada unidad incluya concepto,
problema, alternativas, invariantes, Mermaid, ejemplos, ejercicios,
soluciones, límites, referencias y trazabilidad GitHub. No deben quedar issues
accionables, PRs ni milestones abiertos; `main` debe estar limpia y validada.
