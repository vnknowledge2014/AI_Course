---
description: Universal standards for code quality, functional architecture, and error handling.
trigger: always_on
---
# Core Engineering Standards

1.  **Code Quality**: Write explicit, readable, modular code. Avoid magic numbers and implicit side effects.
    _Rationale: Explicit code survives context loss between agent sessions._
2.  **Functional Architecture**: Immutability over mutation. Use pure functions. Data flows down, actions flow up.
3.  **Error Handling**: Handle errors as values (`Result`, `Option`, `Either`). Never swallow errors silently. Fail fast, log comprehensively.
    _Rationale: Swallowed errors in sub-agents are invisible to the orchestrator._
4.  **Modularization**: Keep functions small and single-purpose. Separate business logic from I/O and side effects.
5.  **Challenge Assumptions**: Before implementing, question: What assumptions am I making? Where does this logic break under edge cases? Is there a simpler approach I'm overlooking?
    _Rationale: AI defaults to compliance mode — this rule forces critical evaluation before action._
