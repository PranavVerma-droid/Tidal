#!/bin/bash


for file in ./code/Normal/*.td; 
do
    ./td "$file"
done