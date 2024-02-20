@test "query_params_parser success1" {
    run cargo run --quiet tools query-params-parser 'name="alex"&is_outdoor=0&id=123'
    [[ "$output" = '{"id":123,"is_outdoor":0,"name":"alex"}' ]]
}

@test "query_params_parser success2" {
    run cargo run --quiet tools query-params-parser 'field1="value1"&field1="value2"&field2="value3"'
    [[ "$output" = '{"field1":"value2","field2":"value3"}' ]]
}

@test "query_params_parser fail1" {
    run cargo run --quiet tools query-params-parser 'value=asdf'
    [[ "$output" = 'query_param_parser::main: input has wrong format' ]]
}

@test "query_params_parser fail2" {
    run cargo run --quiet tools query-params-parser 'ok=true'
    [[ "$output" = 'query_param_parser::main: input has wrong format' ]]
}
