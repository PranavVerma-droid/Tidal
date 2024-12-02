#!/bin/bash

for file in ../../Examples/Normal/*.td; 
do
    echo "Processing file: $(basename "$file")"
    ../td "$file"
done