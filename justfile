test-cli:
    bats --jobs $(nproc) --verbose-run --recursive ./tests