#!/bin/sh

PREF="one. two. three. four. five. six. seven. eight. nine. ten."

read parent_wallet

for i in $PREF; do
    # echo $i$parent_wallet
    near create-account $i$parent_wallet --masterAccount $parent_wallet --initialBalance 10
done