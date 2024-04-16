# musicbrainz-artist-scoring

This repository is used to compute a 5-star rating for the MusicBrainz song dataset. You can read the formula in the code.

## Usage

1. First compile the program in release.

```bash
cargo build --release
```

2. Once you [downloaded the NDJSON MusicBrainz dataset][mb-download-link] you can pass it a first time in this program to output documents with a raw-rated field.

```bash
gzcat ~/Downloads/musicbrainz.ndjson.gz | ./target/release/musicbrainz-artist-scoring | gzip | cpipe -vt > ~/Downloads/musicbrainz-rated.ndjson.gz
```

3. After this you'll need to run a small script that extracts the raw-rated scores for the top musics of the dataset. This script will output the `BEST_SCORE` and `SMALLEST_TOP_SCORE` that you'll need to modify in the code.

```bash
./compute-best-scores.sh
```

4. Once you modified then, please go to step 1 and stop before step 3. You can use the latest dataset you just got with a great 5-star rating included 🎉

[mb-download-link]: https://www.notion.so/meilisearch/Songs-from-MusicBrainz-686e31b2bd3845898c7746f502a6e117
