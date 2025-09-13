#!/bin/bash
#
# Utility functions for git hook scripts.

if test -t 1 && test -n "$(tput colors)" && test "$(tput colors)" -ge 8; then
    bold="$(tput bold)"
    normal="$(tput sgr0)"
    green="$(tput setaf 2)"
    red="$(tput setaf 1)"
    blue="$(tput setaf 4)"

    function hook_failure {
        echo "${red}${bold}FAILED:${normal} ${1}${normal}"
        exit 1
    }

    function hook_info {
        echo "${blue}${1}${normal}"
    }

    function hook_success {
        echo "${green}${bold}SUCCESS:${normal} ${1}${normal}"
        echo
        echo
    }

else
    function hook_failure {
        echo "FAILED: ${1}"
        exit 1
    }

    function hook_info {
        echo "{$1}"
    }

    function hook_success {
        echo "SUCCESS: ${1}"
        echo
        echo
    }
fi

function ensure_rustup_installed() {
    hook_info "📦️ Ensuring that rustup is installed"
    if ! which rustup &> /dev/null; then
        curl https://sh.rustup.rs -sSf  | sh -s -- -y
        export PATH=$PATH:$HOME/.cargo/bin
        if ! which rustup &> /dev/null; then
            hook_failure "Failed to install rustup"
        else
            hook_success "rustup installed."
        fi
    else
        hook_success "rustup is already installed."
    fi
}

function ensure_rustfmt_installed() {
    hook_info "📦️ Ensuring that nightly rustfmt is installed"
    if ! rustup component list --toolchain nightly|grep 'rustfmt-preview.*(installed)' &> /dev/null; then
        rustup component add rustfmt-preview --toolchain nightly
        hook_success "rustfmt installed."
    else
        hook_success "rustfmt is already installed."
    fi
}

function ensure_clippy_installed() {
    hook_info "📦️ Ensuring that clippy is installed"
    if ! rustup component list --toolchain stable|grep 'clippy.*(installed)' &> /dev/null; then
        rustup component add clippy
        hook_success "clippy installed."
    else
        hook_success "clippy is already installed."
    fi
}

function check_formatting() {
    hook_info "🎨 Running 'cargo +nightly fmt -- --check'"
    cargo +nightly fmt -- --check \
        && hook_success "Project is formatted" \
        || hook_failure "Cargo format detected errors."
}

function check_clippy() {
    hook_info "🔍 Running 'cargo clippy -- -D warnings'"
    cargo clippy -- -D warnings \
        && hook_success "Clippy detected no issues" \
        || hook_failure "Cargo clippy detected errors."
}

function check_unused_dependencies() {
    hook_info "🧹 Checking for unused dependencies with `cargo machete`."
    if ! cargo machete --version &> /dev/null; then
        hook_info "Installing cargo machete.."
        cargo install cargo-machete
        hook_info "cargo machete installed."
    fi

    cargo machete --skip-target-dir
    if test $? = 1; then
        hook_failure "At least one unused (non-ignored) dependency found. Please run 'cargo machete fix' to remove them."
    elif test $? = 2; then
        hook_failure "An error occurred while checking for unused dependencies."
    else
        hook_success "No unused dependencies found."
    fi
}
