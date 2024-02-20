@test "primes_up_to(0)" {
    run cargo run --quiet algorithms primes-up-to 0
    [[ "$output" = "[]" ]]
}

@test "primes_up_to(1)" {
    run cargo run --quiet algorithms primes-up-to 1
    [[ "$output" = "[]" ]]
}

@test "primes_up_to(2)" {
    run cargo run --quiet algorithms primes-up-to 2
    [[ "$output" = "[2]" ]]
}

@test "primes_up_to(10)" {
    run cargo run --quiet algorithms primes-up-to 10
    [[ "$output" = "[2, 3, 5, 7]" ]]
}

@test "primes_up_to(100)" {
    run cargo run --quiet algorithms primes-up-to 100
    [[ "$output" = "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]" ]]
}

@test "primes_up_to(101)" {
    run cargo run --quiet algorithms primes-up-to 101
    [[ "$output" = "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101]" ]]
}

@test "primes_up_to(120)" {
    run cargo run --quiet algorithms primes-up-to 120
    [[ "$output" = "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113]" ]]
}
