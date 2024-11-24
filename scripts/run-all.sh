#!/bin/bash


for file in ../Examples/Normal/*.td; 
do
    ./td "$file"
done