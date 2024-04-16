#!/usr/bin/env bash

set -e

# This computes the raw_rating & rating
# However, we ignore the rating one
gzcat musicbrainz-with-raw-rating.ndjson.gzip | jq -r '.raw_rating' | sort -rn | head -n 500000 > largest_values.txt

# We now compute the
cat largest_values.txt | uniq > uniq-largest-values.txt

echo "Use this for the BEST_SCORE variable" $(head -1 uniq-largest-values.txt)
echo "Use this for the SMALLEST_TOP_SCORE variable" $(tail -1 uniq-largest-values.txt)
