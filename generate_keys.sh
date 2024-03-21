mkdir keys
openssl ecparam -name prime256v1 -genkey -noout -out keys/private_key.pem
openssl ec -in keys/private_key.pem -pubout -outform DER 2>&1 | tail -c 65 | base64 | tr '/+' '_-' | tail -n 1 > keys/public_key.txt

# ensure the keys have the correct permissions
chmod 600 keys/private_key.pem
chmod 644 keys/public_key.txt