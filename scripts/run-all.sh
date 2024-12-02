#!/bin/bash

ignore_files=("")

for file in ../../Examples/Normal/*.td; do
    basename=$(basename "$file")
    
    skip=false
    for ignore in "${ignore_files[@]}"; do
        if [[ "$basename" == "$ignore" ]]; then
            skip=true
            break
        fi
    done

    if [[ "$skip" == true ]]; then
        echo "Skipping ignored file: $basename"
        continue
    fi

    echo "Processing file: $basename"
    ../td "$file" -v
done