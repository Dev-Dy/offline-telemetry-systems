#!/bin/sh

echo "Installing git hooks..."

HOOKS_DIR=".git/hooks"

mkdir -p $HOOKS_DIR

cp scripts/pre-commit.sh $HOOKS_DIR/pre-commit

chmod +x $HOOKS_DIR/pre-commit

echo "✅ Pre-commit hook installed"
