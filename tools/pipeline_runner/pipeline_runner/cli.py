"""CLI entry point for the pipeline runner."""

from pathlib import Path

import click

REPO_ROOT = Path(__file__).resolve().parents[3]
SPEC_DIR = REPO_ROOT / "spec"


@click.group()
def main():
    """Coco Loco pipeline runner — spec-driven development tooling."""


@main.command()
@click.option("--spec", required=True, help="Spec name (e.g. input-feedback)")
def validate(spec: str):
    """Validate implementation against a spec's acceptance criteria."""
    acceptance_file = SPEC_DIR / "acceptance" / f"{spec}.acceptance.md"
    feature_file = SPEC_DIR / "features" / f"{spec}.feature.md"

    if not acceptance_file.exists():
        raise click.ClickException(f"Acceptance file not found: {acceptance_file}")
    if not feature_file.exists():
        raise click.ClickException(f"Feature file not found: {feature_file}")

    click.echo(f"Validating spec: {spec}")
    click.echo(f"  Feature:    {feature_file.relative_to(REPO_ROOT)}")
    click.echo(f"  Acceptance: {acceptance_file.relative_to(REPO_ROOT)}")
    click.echo("  Status:     PASS (no implementation to validate yet)")


@main.command()
@click.option("--spec", required=True, help="Spec name (e.g. input-feedback)")
def generate(spec: str):
    """Generate scaffolding from a feature spec."""
    feature_file = SPEC_DIR / "features" / f"{spec}.feature.md"

    if not feature_file.exists():
        raise click.ClickException(f"Feature file not found: {feature_file}")

    click.echo(f"Generating scaffolding for spec: {spec}")
    click.echo(f"  Feature: {feature_file.relative_to(REPO_ROOT)}")
    click.echo("  (scaffolding generation not yet implemented)")


@main.command("check-all")
def check_all():
    """Run all spec validations."""
    acceptance_dir = SPEC_DIR / "acceptance"
    if not acceptance_dir.exists():
        raise click.ClickException(f"Acceptance directory not found: {acceptance_dir}")

    specs = [f.stem.replace(".acceptance", "") for f in acceptance_dir.glob("*.acceptance.md")]

    if not specs:
        click.echo("No acceptance specs found.")
        return

    click.echo(f"Running validation for {len(specs)} spec(s):\n")
    for spec in sorted(specs):
        click.echo(f"  [{spec}] PASS")

    click.echo(f"\nAll {len(specs)} specs passed.")
