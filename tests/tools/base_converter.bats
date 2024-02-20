@test "base_converter finished" {
    run bash -c 'echo "3
13
asdf
2
asdf
123
0011
3
13
asdf
10" | cargo run --quiet tools base-converter'

    [[ "$output" = "Enter the original base (supported bases: [2, 10, 16]): invalid base!
Enter the original base (supported bases: [2, 10, 16]): invalid base!
Enter the original base (supported bases: [2, 10, 16]): invalid base!
Enter the original base (supported bases: [2, 10, 16]): Enter the number: please enter a valid positive integer in the selected base!: invalid digit found in string
Enter the number: please enter a valid positive integer in the selected base!: invalid digit found in string
Enter the number: Enter the new base (supported bases: [2, 10, 16]): invalid base!
Enter the new base (supported bases: [2, 10, 16]): invalid base!
Enter the new base (supported bases: [2, 10, 16]): invalid base!
Enter the new base (supported bases: [2, 10, 16]): 0011 (base 2) = 3 (base 10)" ]]
}
