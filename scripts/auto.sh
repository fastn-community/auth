export PROJ_ROOT=$(pwd)

export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
export DATABASE_URL=${DATABASE_URL:-postgresql://127.0.0.1/fifthtry}

function pushd2() {
    PUSHED=$(pwd)
    cd "${PROJDIR}""$1" >> /dev/null || return
}

function popd2() {
    cd "${PUSHED:-$PROJDIR}" >> /dev/null || return
    unset PUSHED
}

function build-mobile-wasm() {
    pushd2 "${PROJ_ROOT}/mobile-auth-provider" || return 1
    # cargo clean
    cargo build --target wasm32-unknown-unknown --release || return 1
    cp ../target/wasm32-unknown-unknown/release/mobile_auth_provider.wasm . || return 1
    popd2
}


function build-email-wasm() {
    pushd2 "${PROJ_ROOT}/email-auth-provider" || return 1
    # cargo clean
    cargo build --target wasm32-unknown-unknown --release || return 1
    cp ../target/wasm32-unknown-unknown/release/email_auth_provider.wasm . || return 1
    popd2
}

function dotcom() {
  build-email-wasm || return 1
  pushd2 "${PROJ_ROOT}" || return 1
  cp ./email-auth-provider/email_auth_provider.wasm ../dotcom/frontend/email_auth.wasm || return 1
  popd2
}