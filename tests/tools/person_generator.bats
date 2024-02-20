@test "person_generator finished" {
    run cargo run --quiet tools person-generator "name" "2006-02-19" "room" "building" "street" "district" -o /dev/stdout
    [[ "$output" = 'name,age,address
name,18,"room, building, street, district"' ]]
}
