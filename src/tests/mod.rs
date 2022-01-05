use insta::assert_debug_snapshot;
use std::collections::BTreeMap;
use colored::Colorize;

use eyre::{
    eyre,
    // Context as _,
};

pub use super::{
    parse_response,
    Response,
};

fn responses_for(v: &str) -> eyre::Result<Vec<Response>> {
    std::iter::repeat(true)
        .scan(v, |acc, _| {
            let mut try_parse = move || {
                if acc.len() == 0 {
                    return Ok(None)
                }

                let (
                    remaining,
                    (parsed, response)
                ) = parse_response(acc)
                    .map_err(|err| {
                        eyre!(
                            "[{}]\n Error: {:?}. Input: {:?}\n",
                            "ERROR".red(),
                            err,
                            acc,
                        )
                    })?;

                if let Response::Unknown = response {
                    Err(eyre!(
                        "[{}]\nParsed: {:?} . Input: {:?}\n",
                        "UNKNOWN RESPONSE".red(),
                        parsed,
                        acc,
                    ))?;
                };

                *acc = remaining;

                Ok(Some(response))
            };

            try_parse().transpose()
        })
        .collect::<eyre::Result<Vec<Response>>>()
}

fn snapshot_test_responses(data: &str) -> eyre::Result<()> {
    let data: BTreeMap<String, BTreeMap<String, String>> = toml::from_str(data)?;

    type SectionResponseTree = BTreeMap<String, Vec<Response>>;
    type Snapshot = BTreeMap<String, SectionResponseTree>;

    let responses: eyre::Result<Snapshot> = data
        .iter()
        .map(|(section_title, scenarios)| {
            let section_title = section_title.clone();

            let section_responses = scenarios.clone()
                .iter()
                .map(|(k, v)| {
                    let title = format!("{}::{}", &section_title, k);

                    print!("\nTesting {:?} ", title);
                    let res = responses_for(&v)?;

                    println!("[{}]", "DONE".green());
                    println!("Got: {:?}\n", res);

                    Ok((k.to_owned(), res))
                })
                .collect::<eyre::Result<SectionResponseTree>>()?;

            Ok((section_title, section_responses))
        })
        .collect();

    assert_debug_snapshot!(responses?);

    Ok(())
}

#[test]
fn ender3_marlin_2019_firmware() -> eyre::Result<()> {
    let data = include_str!("data/ender3_marlin_2019_firmware.toml");
    snapshot_test_responses(data)
}

#[test]
fn ultimaker2_marlin_dbg_2019_firmware() -> eyre::Result<()> {
    let data = include_str!("data/ultimaker2_marlin_dbg_2019_firmware.toml");
    snapshot_test_responses(data)
}
