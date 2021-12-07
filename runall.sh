#!/bin/bash

cargo build

for ((i=1; i<=$1; i++))
do
	"target/debug/day$i"
done

