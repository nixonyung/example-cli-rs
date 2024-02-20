use super::envs::Envs;
use anyhow::Context;
use rand::Rng;

pub fn main() -> anyhow::Result<()> {
    let envs = envy::from_env::<Envs>().context("error reading envs")?;

    let ans = match envs.ans {
        Some(ans) => {
            anyhow::ensure!(
                envs.ans_min <= ans && ans <= envs.ans_max,
                "config error: ANS ({}) should be between ANS_MIN ({}) and ANS_MAX ({})",
                ans,
                envs.ans_min,
                envs.ans_max
            );
            ans
        }
        None => {
            let mut rng = rand::thread_rng();
            rng.gen_range(envs.ans_min..=envs.ans_max)
        }
    };
    let mut curr_min = envs.ans_min;
    let mut curr_max = envs.ans_max;
    loop {
        let input = loop {
            match (|| -> _ {
                let input = common::input(
                    format!("Enter a positive integer between {curr_min} and {curr_max}: ")
                        .as_str(),
                )?;
                let parsed = match input.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => anyhow::bail!("you did not enter a positive integer!"),
                };
                anyhow::ensure!(
                    curr_min <= parsed && parsed <= curr_max,
                    "please enter a positive integer between {curr_min} and {curr_max}!"
                );

                Ok(parsed)
            })() {
                Ok(val) => break val,
                Err(err) => common::print_err(err),
            }
        };
        if input == ans {
            println!("YOU WIN!!!");
            break;
        }
        if input < ans {
            println!("too small");
            curr_min = input + 1;
        } else {
            println!("too big");
            curr_max = input - 1;
        }
    }
    Ok(())
}
