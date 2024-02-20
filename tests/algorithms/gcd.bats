@test "gcd(3, 1)" {
    run cargo run --quiet algorithms gcd 3 1
    [[ "$output" = "gcd of 3 and 1 = 1" ]]
}

@test "gcd(1, 3)" {
    run cargo run --quiet algorithms gcd 1 3
    [[ "$output" = "gcd of 1 and 3 = 1" ]]
}

@test "gcd(5, 5)" {
    run cargo run --quiet algorithms gcd 5 5
    [[ "$output" = "gcd of 5 and 5 = 5" ]]
}

@test "gcd(20, 6)" {
    run cargo run --quiet algorithms gcd 20 6
    [[ "$output" = "gcd of 20 and 6 = 2" ]]
}

@test "gcd(6, 20)" {
    run cargo run --quiet algorithms gcd 6 20
    [[ "$output" = "gcd of 6 and 20 = 2" ]]
}

@test "gcd(100, 52)" {
    run cargo run --quiet algorithms gcd 100 52
    [[ "$output" = "gcd of 100 and 52 = 4" ]]
}

@test "gcd(52, 100)" {
    run cargo run --quiet algorithms gcd 52 100
    [[ "$output" = "gcd of 52 and 100 = 4" ]]
}
