#!/bin/bash
# call this script with an email address (valid or not).
# like:
# ./makecert.sh joe@random.com
mkdir -p credentials/develop/
rm credentials/develop/*

echo "make server cert"
openssl req -new -nodes -x509 -out credentials/develop/server.pem -keyout credentials/develop/server.key -days 3650 -subj "/C=DE/ST=NRW/L=Earth/O=Localhost Company/OU=IT/CN=127.0.0.1/emailAddress=$1"

echo "make client cert"
openssl req -new -nodes -x509 -out credentials/develop/client.pem -keyout credentials/develop/client.key -days 3650 -subj "/C=DE/ST=NRW/L=Earth/O=Localhost Company/OU=IT/CN=127.0.0.1/emailAddress=$1"
