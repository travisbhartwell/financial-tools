#!/usr/bin/env bash
# -*- mode: shell-script; sh-shell: bash; sh-basic-offset: 4; sh-indentation: 4; coding: utf-8 -*-
# shellcheck shell=bash

# This is going to be the simplest possible thing that could work
# until the full version is in MyCmd itself.

set -o nounset -o errexit -o errtrace -o pipefail

if ! PROJECT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P); then
    echo >&2 "Error fetching project directory."
    exit 1
fi

readonly DIST_DIR="${PROJECT_DIR}/dist"

function build() {
    hatch build
}

function lint-all() {
    hatch run lint:all
}

function fmt() {
    hatch run lint:fmt
}

function install() {
    local -r wheel="${DIST_DIR}/datacleanup-0.0.1-py3-none-any.whl"

    build

    if [[ ! -e "${wheel}" ]]; then
        echo >&2 "Missing wheel ${wheel}."
        return 1
    fi

    pipx install --include-deps "${wheel}"
}

function function_exists() {
    declare -F "$1" >/dev/null
}

function call_if_function_exists() {
    local -r fn=$1
    shift

    if function_exists "${fn}"; then
        echo >&2 "Executing task: ${fn}..."
        "${fn}" "$@"
    else
        echo >&2 "Unknown task: '${fn}'."
    fi
}

if (($# == 0)); then
    echo >&2 "Expecting task to run:"
    echo >&2 "$0 <task>"
    exit 1
fi

call_if_function_exists "${@}"
