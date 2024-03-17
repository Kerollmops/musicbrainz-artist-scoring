# musicbrainz-artist-scoring

```bash
cargo build --release

gzcat ~/Downloads/musicbrainz.ndjson.gz | ./target/release/musicbrainz-artist-scoring | gzip | cpipe -vt > ~/Downloads/musicbrainz-rated.ndjson.gz
```
