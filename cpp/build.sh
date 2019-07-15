#!/bin/bash
dir="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
clang++ -O2 -Wall -std=c++14 "${dir}/main.cc" -o "${dir}/pmf_cpp"
