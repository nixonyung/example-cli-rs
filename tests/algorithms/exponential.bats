@test "exponential(10, 0)" {
    run cargo run --quiet algorithms exponential 10 0
    [[ "$output" = "10^0 = 1" ]]
}

@test "exponential(10, 2)" {
    run cargo run --quiet algorithms exponential 10 2
    [[ "$output" = "10^2 = 100" ]]
}

@test "exponential(2, 10)" {
    run cargo run --quiet algorithms exponential 2 10
    [[ "$output" = "2^10 = 1024" ]]
}
