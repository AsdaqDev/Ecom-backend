#!/usr/bin/env bash 

set -euo pipefail

echo "creating modules structure"

mkdir -p \
    src/modules/auth/adapter/postgres \
    src/modules/auth/adapter/redis \
    src/modules/auth/adapter/jwt \
    src/modules/auth/adapter/password \
    src/modules/auth/adapter/axum \
    src/modules/auth/domain/value_obj \
    src/modules/auth/feature/login \
    src/modules/auth/feature/register \
    src/modules/auth/feature/forget_pass \
    src/modules/auth/feature/refresh_token \

touch \
    src/modules/auth/adapter/postgres/repo.rs \
    src/modules/auth/adapter/password/argon.rs \
    src/modules/auth/adapter/jwt/jwt.rs \
    src/modules/auth/adapter/redis/redis.rs \
    src/modules/auth/adapter/axum/route.rs \
    src/modules/auth/adapter/axum/handler.rs \
    src/modules/auth/adapter/axum/error.rs \

echo "stucture created successfully"
