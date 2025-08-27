#!/bin/bash

set -e
set -u

echo "Reading version from Cargo.toml..."
VERSION=$(grep '^version = ' Cargo.toml | sed -e 's/version = "\(.*\)"/\1/')
if [ -z "$VERSION" ]; then
    echo "Error: Could not find version in Cargo.toml"
    exit 1
fi
echo "Found version: $VERSION"

TAG_NAME="leptos-windowing-v$VERSION"
echo "Constructed tag name: $TAG_NAME"

echo "Deleting local tag $TAG_NAME (if it exists)..."
git tag -d "$TAG_NAME" || true

echo "Deleting remote tag $TAG_NAME from origin (if it exists)..."
git push --delete origin "$TAG_NAME" || true

echo "Creating new tag $TAG_NAME on HEAD..."
git tag "$TAG_NAME" HEAD

echo "Pushing current branch's latest commit to default upstream..."
git push

echo "Pushing new tag $TAG_NAME to origin..."
git push origin "$TAG_NAME"

echo "Script completed successfully."
