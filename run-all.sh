#!/usr/bin/zsh

for quine in quines/*; do
    echo $quine
    runner=$(echo $quine | rev | cut -d. -f1 | rev)
    runners/$runner $quine | diff $quine -
done
