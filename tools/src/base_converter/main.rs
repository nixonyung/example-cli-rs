use super::base::Base;
use anyhow::Context;

pub fn main() {
    let original_base = loop {
        match (|| -> _ {
            let input = common::input(
                format!(
                    "Enter the original base (supported bases: {:?}): ",
                    [2, 10, 16]
                )
                .as_str(),
            )?;
            Base::try_from(input.as_str())
        })() {
            Ok(val) => break val,
            Err(err) => common::print_err(err),
        }
    };
    let (number_str, number) = loop {
        match (|| -> _ {
            let input = common::input("Enter the number: ")?;
            let val = u32::from_str_radix(input.as_str(), original_base.into())
                .context("please enter a valid positive integer in the selected base!")?;
            Ok((input, val))
        })() {
            Ok((input, val)) => break (input, val),
            Err(err) => common::print_err(err),
        }
    };
    let new_base = loop {
        match (|| -> _ {
            let input = common::input(
                format!("Enter the new base (supported bases: {:?}): ", [2, 10, 16]).as_str(),
            )?;
            Base::try_from(input.as_str())
        })() {
            Ok(val) => break val,
            Err(err) => common::print_err(err),
        }
    };

    print!("{number_str} (base {original_base}) = ");
    match new_base {
        Base::BINARY => print!("{number:b}"),
        Base::DECIMAL => print!("{number}"),
        Base::HEXADECIMAL => print!("{number:x}"),
    };
    println!(" (base {new_base})")
}
