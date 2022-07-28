up_postgres="docker run -p 3000:5432 -e POSTGRES_USER=dbuser -e POSTGRES_PASSWORD=dbpassword -e POSTGRES_DB=dbexample -d postgres"
eval $up_postgres

run_migrations="docker run --name migrations --network=host migrator"
eval $run_migrations

run_api="docker run --name api -p 8080:8080 --network=host warp-api"
eval $run_api