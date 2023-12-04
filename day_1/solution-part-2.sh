#!/bin/env sh

cat input.txt |\
  sed 's/[^0-9]//g' |\
  sed -r 's/^(.).*(.)$/\1\2/' |\
  sed -r 's/^(.)$/\1\1/' |\
  xargs |\
  tr ' ' '+' |\
  bc
