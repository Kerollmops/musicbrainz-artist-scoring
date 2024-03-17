use std::io::{self, BufReader, BufWriter};

use big_s::S;
use calm_io::*;
use serde_json::de::Deserializer;
use serde_json::{json, Map, Value};

#[pipefail]
fn main() -> io::Result<()> {
    let stdin = BufReader::new(io::stdin());
    let mut stdout = BufWriter::new(io::stdout());

    for result in Deserializer::from_reader(stdin).into_iter() {
        let mut document: Map<String, Value> = result?;

        if let Some(artist_rating) = document.get("artist_rating").and_then(|v| v.as_u64()) {
            // create five groups, from 0 to 4
            document.insert(S("rating"), json!(artist_rating * 4 / 100));
        }

        serde_json::to_writer(&mut stdout, &document)?;
    }

    Ok(())
}
