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
            let rating = match artist_rating {
                0..=40 => 0,
                41..=70 => 1,
                71..=80 => 2,
                81..=90 => 3,
                91.. => 4,
            };

            document.insert(S("rating"), json!(rating));
        }

        serde_json::to_writer(&mut stdout, &document)?;
    }

    Ok(())
}
