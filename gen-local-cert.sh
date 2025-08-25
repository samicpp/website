openssl genrsa -out localhost.key 2048
openssl req -new -key localhost.key -out localhost.csr
openssl x509 -req -days 365 -in localhost.csr -signkey localhost.key -out localhost.crt
openssl pkcs12 -export -inkey localhost.key -in localhost.crt -out identity.p12 -name "localhost"
# ln -s localhost.key key.pem
# ln -s localhost.crt cert.pem
