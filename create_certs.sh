eval "mkdir -p tls && cd tls"

new_key="openssl genrsa -out key.pem"
eval $new_key

new_csr="openssl req -new -key key.pem -out csr.pem"
eval $new_csr

gen_cert="openssl x509 -req -days 9999 -in csr.pem -signkey key.pem -out cert.pem"
eval $gen_cert

remove_csr="rm csr.pem"
eval $remove_csr
