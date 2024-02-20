use anyhow::Context;

#[derive(serde::Serialize, Debug)]
struct PersonView<'a> {
    pub name: &'a str,
    pub age: &'a u32,
    pub address: &'a str,
}

pub fn main(
    name: String,
    birthday_str: String,
    address_room: String,
    address_building: String,
    address_street: String,
    address_district: String,
    out_path_str: String,
) -> anyhow::Result<()> {
    let person = super::person::Person {
        name,
        birthday: chrono::NaiveDate::parse_from_str(birthday_str.as_str(), "%Y-%m-%d")
            .context("parsing birthday")?,
        address: super::person::Address {
            room: address_room,
            building: address_building,
            street: address_street,
            district: address_district,
        },
    };

    let out_path = std::path::Path::new(&out_path_str);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).context(format!("creating dir {:?}", parent))?;
    }

    let mut writer =
        csv::Writer::from_path(out_path).context("creating writer to ./out/out.csv")?;
    writer
        .serialize(PersonView {
            name: person.name.as_str(),
            age: &person.age(),
            address: {
                [
                    person.address.room.as_str(),
                    person.address.building.as_str(),
                    person.address.street.as_str(),
                    person.address.district.as_str(),
                ]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(", ")
            }
            .as_str(),
        })
        .context(format!("writing a PersonView to {:?}", out_path))?;
    Ok(())
}
