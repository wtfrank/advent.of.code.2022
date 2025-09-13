#!/usr/bin/env bash

# this script sets up the next day's crate by copying
# from the previous day's crate.

if [[ $# != 1 ]]
then
  echo Usage: $0 \<DAY OF ADVENT\>
  echo For example: $0 5
  exit 1
fi

if [[ ! $1 =~ ^[0-9]+$ ]]
then
  echo day should just be a number: $1
  exit 1
fi

PREV_DIR=day$(($1 -1))
NEXT_DIR=day$1

if [[ ! -d $PREV_DIR ]]
then
  echo "Previous day's directory ${PREV_DIR} does not exist"
  exit 1
fi

if [[ -d $NEXT_DIR ]]
then
  echo "Next day's directory ${NEXT_DIR} already exists"
  exit 1
fi

cp -a $PREV_DIR $NEXT_DIR
cd $NEXT_DIR
rm *.txt

sed -E -i 's/input[0-9]+.txt/input'$1'.txt/g' benches/bench.rs src/main.rs
sed -E -i 's/_day[0-9]+"/_day'$1'"/g' Cargo.toml
sed -E -i 's/Day [0-9]+ of Advent/Day '$1' of Advent/g' src/main.rs
sed -E -i 's/_day[0-9]+::/_day'$1'::/g' src/main.rs benches/bench.rs

cd ..
YEAR=$(basename $(pwd))
export YEAR
cd $NEXT_DIR
../../get_puzzle_input.sh $1
