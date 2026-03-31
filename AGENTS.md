# Agent Workflow — Spec-Driven Development

This project follows spec-driven development. Every feature, system, and behaviour is defined in the `spec/` folder before implementation begins. Agents should treat specs as the source of truth.

## Spec Folder Structure

```
spec/
  requirements.md        # High-level product requirements and constraints
  architecture.md        # System architecture and module boundaries
  features/              # One file per feature spec
    *.feature.md
  schemas/               # Data schemas and type contracts
    *.schema.md
  acceptance/            # Acceptance criteria and test scenarios
    *.acceptance.md
```

## Agent Responsibilities

### 1. Planning Phase
- Read `spec/requirements.md` and `spec/architecture.md` to understand scope and constraints.
- Identify which feature specs in `spec/features/` are relevant to the task.
- Check `spec/schemas/` for data contracts that constrain implementation.

### 2. Implementation Phase
- Implement strictly according to the matching feature spec.
- If a spec is ambiguous or missing, flag it — do not invent behaviour.
- Cross-reference `spec/acceptance/` to understand what "done" looks like.

### 3. Validation Phase
- Run acceptance scenarios from `spec/acceptance/` against the implementation.
- Use the pipeline runner (`tools/pipeline_runner`) to execute validation pipelines:
  ```bash
  cd tools/pipeline_runner && poetry run pipeline validate --spec <spec-name>
  ```
- Report any spec-vs-implementation drift.

## Pipeline Runner

All automation pipelines live in `tools/pipeline_runner` (Python/Poetry, zero-install).

Common commands:
```bash
poetry run pipeline validate --spec <name>   # Validate implementation against spec
poetry run pipeline generate --spec <name>   # Generate scaffolding from spec
poetry run pipeline check-all                # Run all spec validations
```

## Adding a New Feature

1. Write the feature spec: `spec/features/<name>.feature.md`
2. Define acceptance criteria: `spec/acceptance/<name>.acceptance.md`
3. Add any new data contracts: `spec/schemas/<name>.schema.md`
4. Implement the feature in Rust/Rhai.
5. Run `poetry run pipeline validate --spec <name>` to verify.
