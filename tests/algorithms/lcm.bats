@test "lcm(3, 1)" {
    run cargo run --quiet algorithms lcm 3 1
    [[ "$output" = "lcm of 3 and 1 = 3" ]]
}

@test "lcm(1, 3)" {
    run cargo run --quiet algorithms lcm 1 3
    [[ "$output" = "lcm of 1 and 3 = 3" ]]
}

@test "lcm(5, 5)" {
    run cargo run --quiet algorithms lcm 5 5
    [[ "$output" = "lcm of 5 and 5 = 5" ]]
}

@test "lcm(20, 6)" {
    run cargo run --quiet algorithms lcm 20 6
    [[ "$output" = "lcm of 20 and 6 = 60" ]]
}

@test "lcm(6, 20)" {
    run cargo run --quiet algorithms lcm 6 20
    [[ "$output" = "lcm of 6 and 20 = 60" ]]
}

@test "lcm(100, 52)" {
    run cargo run --quiet algorithms lcm 100 52
    [[ "$output" = "lcm of 100 and 52 = 1300" ]]
}

@test "lcm(52, 100)" {
    run cargo run --quiet algorithms lcm 52 100
    [[ "$output" = "lcm of 52 and 100 = 1300" ]]
}
