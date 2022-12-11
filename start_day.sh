#!/usr/bin/env bash
cp -n benches/01.rs benches/$1.rs
cp -n src/bin/01.rs src/bin/$1.rs
cp -n src/day01.rs src/day$1.rs

curl \
  --cookie "session=`cat .cookie`"\
  -A "Input fetch for daniel.a.kirkham@gmail.com"\
  https://adventofcode.com/2022/day/$1/input > input/$1.txt

echo "pub mod day$1;" >> src/lib.rs
sed -i "s/01/$1/g" benches/$1.rs
sed -i "s/01/$1/g" src/bin/$1.rs
nvim src/day$1.rs
