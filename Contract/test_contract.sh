#!/bin/bash

PREF="one. two. three. four. five. six. seven. eight. nine. ten."


add_user() {
    near call $1 set_user --args '{"name": "test", "discord": "test.discord", "email": "test.mail", "interests": 1}' --accountId $2
    for i in {0..20..1}; do
        near call $1 set_experience --args '{"experience_name": "test exp", "description": "descripcion del test", "url": "https://test.exp", "reward": 0.1, "moment": "3 moment", "time": 30, "expire_date": 1655314845, "topic": 1}' --accountId $2 --deposit 0.11
    done
}

add_pov() {
    near call $1 set_pov --args '{"video_n": '$3', "pov": "testning pov", "date": 1500}' --accountId $2
}

pay_reward() {
    near call $1 pay_reward --args '{"experience_number": $3, "wallet": \"$4\"}' --accountId $2
}

for i in $PREF; do
    i+=$2
    add_user $1 $i
done

# N_EXP=`near view $1 get_number_of_experiences | grep -v View `
# for (( i = 1; i <= $N_EXP; i++ ))
# do
#     for j in $PREF
#     do
#         j+=$2
#         add_pov $1 $j $i
#     done
# donewallet