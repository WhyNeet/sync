#!/bin/sh
for i in $(env | grep SYNC_)
do
    key=$(echo $i | cut -d '=' -f 1)
    value=$(echo $i | cut -d '=' -f 2-)
    echo $key=$value
    # sed All files
    # find /usr/share/nginx/html -type f -exec sed -i "s|${key}|${value}|g" '{}' +

    find /usr/share/nginx/html -type f \( -name 'env*.js' \) -exec sed -i "s/${key}/${value}/g" '{}' +
done