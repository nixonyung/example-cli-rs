use once_cell::sync::Lazy;
use regex::Regex;

static QUERY_PARAMS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^\\w+=(\"[[:print:]]*\"|\\d+)(&\\w+=(\"[[:print:]]*\"|\\d+))*$").unwrap()
});

pub fn main(input: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        QUERY_PARAMS_PATTERN.is_match(input),
        "input has wrong format"
    );

    println!(
        "{}",
        serde_json::Value::Object(
            input
                .split("&")
                .into_iter()
                .map(|record| record.split_once("=").unwrap())
                .map(|(key, value)| {
                    (
                        key.to_owned(),
                        if value.starts_with("\"") {
                            // value is a string (wrapped in double-quotes)
                            serde_json::Value::String(value[1..value.len() - 1].to_owned())
                        } else {
                            // value is a number
                            serde_json::Value::Number(value.parse::<i32>().unwrap().into())
                        },
                    )
                })
                .collect::<serde_json::Map<String, serde_json::Value>>(),
        ),
    );

    Ok(())
}
