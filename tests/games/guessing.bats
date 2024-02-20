@test "guessing game finished" {
    run bash -c 'echo "-1
101
asdf
50
25
37
45
40
42" | ANS=42 cargo run --quiet games guessing'

    [[ "$output" = "Enter a positive integer between 1 and 100: you did not enter a positive integer!
Enter a positive integer between 1 and 100: please enter a positive integer between 1 and 100!
Enter a positive integer between 1 and 100: you did not enter a positive integer!
Enter a positive integer between 1 and 100: too big
Enter a positive integer between 1 and 49: too small
Enter a positive integer between 26 and 49: too small
Enter a positive integer between 38 and 49: too big
Enter a positive integer between 38 and 44: too small
Enter a positive integer between 41 and 44: YOU WIN!!!" ]]
}
