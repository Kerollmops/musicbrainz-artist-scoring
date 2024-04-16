use std::io::{self, BufReader, BufWriter};

use big_s::S;
use calm_io::*;
use serde_json::de::Deserializer;
use serde_json::{json, Map, Value};

const BEST_SCORE: u64 = 739200;
const SMALLEST_TOP_SCORE: u64 = 93960;

#[pipefail]
fn main() -> io::Result<()> {
    let stdin = BufReader::new(io::stdin());
    let mut stdout = BufWriter::new(io::stdout());

    for result in Deserializer::from_reader(stdin).into_iter() {
        let mut document: Map<String, Value> = result?;

        let raw_rating = compute_raw_rating(&document);

        if let Some(raw_rating) = raw_rating {
            document.insert(S("raw_rating"), json!(raw_rating));
            let rating = compute_rating(raw_rating);
            document.insert(S("raw_rating"), json!(raw_rating));
            document.insert(S("rating"), json!(rating));
        }

        if raw_rating.map_or(true, |s| s < SMALLEST_TOP_SCORE) {
            document.insert(S("_vectors"), json!({ "OpenAI": null }));
        }

        if document.get("released_year").and_then(|v| v.as_u64()).is_none() {
            document.insert(S("released_year"), json!(1970));
        }

        serde_json::to_writer(&mut stdout, &document)?;
    }

    Ok(())
}

fn compute_raw_rating(document: &Map<String, Value>) -> Option<u64> {
    let artist_rating = document.get("artist_rating").and_then(|v| v.as_u64())?;
    let artist_rating_count = document.get("artist_rating_count").and_then(|v| v.as_u64())?;
    let track_rating = document.get("track_rating").and_then(|v| v.as_u64()).unwrap_or(1);

    Some(artist_rating * artist_rating_count * track_rating)
}

// smallest top score -> 4/5 stars
// biggest score -> 5/5 stars
fn compute_rating(raw_rating: u64) -> f64 {
    let smallest = SMALLEST_TOP_SCORE / 10;
    let smallest_float = SMALLEST_TOP_SCORE as f64 / 10.0;

    let raw_rating_float = raw_rating as f64;
    if raw_rating >= smallest {
        let remaining = (BEST_SCORE as f64) - smallest_float / 5.0;
        let base = raw_rating_float - smallest_float;
        4.0 + base / remaining
    } else {
        raw_rating_float * 4.0 / smallest_float
    }
}
