<h1 align="center">Basic API</h1>
<h3>Rust starter API</h3><br>
<ul>
  <li>
  <a href="https://github.com/seanmonstar/warp">
    <strong>Warp</strong> main server framework<br>
  </a>
  </li>
  <li>
  <a href="https://diesel.rs/">
    <strong>Diesel</strong> to allow the use of SQL databases<br>
  </a>
  
  </li>
  <li>
  <a href="https://graphql-rust.github.io/">
    <strong>Juniper</strong> to allow the use of GraphQl<br>
  </a>
  </li>
  <li>
  <a href="https://crates.io/crates/jsonwebtoken">
    <strong>JWT</strong> for authentication<br>
  </a>
  </li>
</ul>
<hr><br>
<h3>Requirements</h3>
<ul>
  <li>
    <a href="https://www.rust-lang.org/tools/install">
    <strong>Rust</strong> (preferably the latest version)<br>
    </a>
  </li>
  <li>
    <a href="https://www.postgresql.org/">
    <strong>PostgreSQL</strong>(if opted for a Postgres database)<br>
    </a><br>
    <h4>
    <code>docker run -p dbport:5432 --name testing -e POSTGRES_USER=dbuser -e POSTGRES_PASSWORD=dbpassword -e POSTGRES_DB=dbexample -d postgres
    </code>
    </h4>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>MySQL</strong>(if opted for a Mysql database)<br>
    </a>
  </li>
  <li>
    After the choiced db is up and running start the database with <br>
    <h4>
    <code>
    diesel setup
    </code>
    </h4>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>Cargo Watch</strong>(strongly recommended for easy development)<br>
    </a>
  </li>
  <li>
        After any migration runned change credential table (or any table using a special type) to the follow (add/ remove the custom columnss at pleasure) <br>
    <h4>
    <code>
    table! {<br>
    use diesel::{sql_types::{Nullable,Bool,Text, Timestamptz}, types::{ Int4, Varchar}};<br>
    use crate::core::credentials::LogModelMapping;<br>
    credentials (id) {<br>
        id -> Int4,<br>
        password -> Text,<br>
        email -> Varchar,<br>
        state -> Nullable&#60;Bool&#62;,<br>
        log_model -> LogModelMapping,<br>
        created_at -> Timestamptz,<br>
        updated_at -> Timestamptz,<br>
    }
}
    </code>
    </h4>
  </li>
</ul>
<hr><br>
<h3>Usage</h3>
<ol>
  <li><strong>Clone</strong> this repository<br>
    <h4>
    <code>git clone https://github.com/giornn0/basic-api.git</code>
    </h4>
  </li>
  <li><strong>Create</strong> the .env file<br>
    <h4>
    <code>cp .env.example .env</code>
    </h4>
  </li>
  <li><strong>Open</strong> the .env file and <strong>configure accordingly.</strong>. <br>
  </li>
  <li>If using <strong>MySQL</strong> remembver to modify the type of the <strong>pooled connection.</strong>. <br>
  </li>
  <li>In the cloned directory run<strong></strong><br>
    <code>cargo watch -x "run"</code><small>(if cargo-watch installed)</small> <br>
    <code>cargo run</code>
  </li>
  <li>Try to work the most part of your application <strong>inside application directory</strong><br>
  </li>
  <li>To manage all the <strong>logins</strong>, <strong>roles</strong> and <strong>pagination</strong> defaults you have the file <strong>config</strong>, inside <strong>application directory</strong><br>
  </li>
  <li><strong>Enjoy</strong><br>
  </li>
</ol>
<hr><br>
<h3>Release</h3>
<ol>
  <li><strong>Run</strong> this command<br>
  <h4>
    <code>cargo watch -x "run --release"</code><small>(if cargo-watch installed)</small>
  </h4><br>
  <h4>
    <code>cargo run --release</code>
  </h4>
  </li>
</ol>
<h3>Pending</h3>
<ul>
  <li><strong>Order</strong> for tokens and credential<br>
  </li>
  <li><strong>Config</strong> folder<br>
    <strong>Order</strong> between some customizables traits and enums 
  </li>
</ul>
<h3>TESTING</h3>
 <h4><code>docker build -t api-image -f ./Dockerfile . </code></h4> <br>
 <h4><code>docker run -it -v $(pwd)/migrations:/usr/src/migrations  --name testing rust:1.62-bullseye</code></h4> <br>
 <h4><code>docker run -it -p 8080:8080 --network=host --name api api-image</code></h4> <br>