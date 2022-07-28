<h1 align="center">Basic API</h1>
<h3>Rust starter API</h3><br/>
<ul>
  <li>
  <a href="https://github.com/seanmonstar/warp">
    <strong>Warp</strong> main server framework<br/>
  </a>
  </li>
  <li>
  <a href="https://diesel.rs/">
    <strong>Diesel</strong> to allow the use of SQL databases<br/>
  </a>
  
  </li>
  <li>
  <a href="https://graphql-rust.github.io/">
    <strong>Juniper</strong> to allow the use of GraphQl<br/>
  </a>
  </li>
  <li>
  <a href="https://crates.io/crates/jsonwebtoken">
    <strong>JWT</strong> for authentication<br/>
  </a>
  </li>
</ul>
<hr/><br/>
<h3>Requirements</h3>
<ul>
  <li>
    <a href="https://www.rust-lang.org/tools/install">
    <strong>Rust</strong> (preferably the latest version)<br/>
    </a>
  </li>
  <li>
    <a href="https://www.postgresql.org/">
    <strong>PostgreSQL</strong>(if opted for a Postgres database)<br/>
    </a><br/>
    <h4>
    <code>docker run -p dbport:5432 --name testing -e POSTGRES_USER=dbuser -e POSTGRES_PASSWORD=dbpassword -e POSTGRES_DB=dbexample -d postgres
    </code>
    </h4>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>MySQL</strong>(if opted for a Mysql database)<br/>
    </a>
  </li>
  <li>
    After the choiced db is up and running start the database with <br/>
    <h4>
    <code>
    diesel setup
    </code>
    </h4>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>Cargo Watch</strong>(strongly recommended for easy development)<br/>
    </a>
  </li>
  <li>
        After any migration runned change credential table (or any table using a special type) to the follow (add/ remove the custom columnss at pleasure) <br/>
    
    
    table! {
    use diesel::{sql_types::{Nullable,Bool,Text, Timestamptz}, types::{ Int4, Varchar}};
    use crate::core::credentials::LogModelMapping;
    credentials (id) {
        id -> Int4,
        password -> Text,
        email -> Varchar,
        state -> Nullable&#60;Bool&#62;,
        log_model -> LogModelMapping,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
      }
    }
    
  </li>
</ul>
<hr/><br/>
<h3>Up With Docker</h3>
<ol>
  <li>Create the <strong>API image</strong><br/>
    <h4><code>docker build -t warp-api -f ./Dockerfile . </code></h4> <br/>
  </li>
  <li>Create the <strong>migrator image</strong><br/>
    <h4><code>docker build -t migrator -f ./Diesel.Dockerfile .</code></h4> <br/>
  </li>
  <li>Start <strong>database</strong> <small>(postgresql in this case)</small><br/>
    <h4><code>docker run -p dbport:5432 --name testing -e POSTGRES_USER=dbuser -e POSTGRES_PASSWORD=dbpassword -e POSTGRES_DB=dbexample -d postgres</h4> <br/>
  </li>
  <li>Run <strong>migrations</strong>
    <h4><code>docker run --name migrate --network=host migrator</code></h4>
  </li>
  <li>Run the <strong>API</strong>
    <h4><code>docker run -p 8080:8080 --name api-service --network=host warp-api</code></h4>
  </li>
  <li>
  <strong>Enjoy!</strong>
  </li>
</ol>
<hr/><br/>
<h3>Usage</h3>
<ol>
  <li><strong>Clone</strong> this repository<br/>
    <h4>
    <code>git clone https://github.com/giornn0/basic-api.git</code>
    </h4>
  </li>
  <li><strong>Create</strong> the .env file<br/>
    <h4>
    <code>cp .env.example .env</code>
    </h4>
  </li>
  <li><strong>Open</strong> the .env file and <strong>configure accordingly</strong>. <br/>
  </li>
  <li>If using <strong>MySQL</strong> remembver to modify the type of the <strong>pooled connection</strong>. <br/>
  </li>
  <li>In the cloned directory run<strong></strong><br/>
  <h4>
    <code>cargo watch -x "run"</code><small>(if cargo-watch installed)</small>
  </h4> <br/>
  <h4>
    <code>cargo run</code>
  </h4>
  </li>
  <li>Try to work the most part of your application <strong>inside application directory</strong><br/>
  </li>
  <li>To manage all the <strong>logins</strong>, <strong>roles</strong> and <strong>pagination</strong> defaults you have the file <strong>config</strong>, inside <strong>application directory</strong><br/>
  </li>
  <li><strong>Enjoy</strong><br/>
  </li>
</ol>
<hr/><br/>
<h3>Release</h3>
<ol>
  <li><strong>Run</strong> this command<br/>
  <h4>
    <code>cargo watch -x "run --release"</code><small>(if cargo-watch installed)</small>
  </h4><br/>
  <h4>
    <code>cargo run --release</code>
  </h4>
  </li>
</ol>
<h3>Pending</h3>
<ul>
  <li><strong>Config</strong> folder<br/>
    <strong>Order</strong> between some customizables traits and enums 
  </li>
</ul>
