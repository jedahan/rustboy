# This script takes care of testing your crate

set -ex

# This is the "test phase", tweak it as you see fit
main() {
          cat > Cross.toml <<EOF
[target.x86_64-unknown-linux-gnu]
image = "jedahan/x86_64-unknown-linux-gnu-rustboy"
EOF

    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    cross run --target $TARGET
    cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
