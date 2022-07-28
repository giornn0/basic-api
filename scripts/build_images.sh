api_image="docker build -t warp-api -f ./Dockerfile . "
eval $api_image

migrate_image="docker build -t migrator -f ./Diesel.Dockerfile ."
eval $migrate_image