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
    </a>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>MySQL</strong>(if opted for a Mysql database)<br>
    </a>
  </li>
  <li>
    <a href="https://www.mysql.com/">
    <strong>Cargo Watch</strong>(strongly recommended for easy development)<br>
    </a>
  </li>
    <li>
        After any migration runned change credential table (or any table using a special type) to the follow (add/ remove the custom columnss at pleasure) <br>
    <code>
    table! {
    use diesel::{sql_types::{Nullable,Bool,Text, Timestamptz}, types::{ Int4, Varchar}};
    use crate::core::credentials::LogModelMapping;
    credentials (id) {
        id -> Int4,
        password -> Text,
        email -> Varchar,
        state -> Nullable<Bool>,
        log_model -> LogModelMapping,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
    </code>
  </li>
</ul>
<hr><br>
<h3>Usage</h3>
<ol>
  <li><strong>Clone</strong> this repository<br>
    <code>git clone https://github.com/giornn0/basic-api.git</code>
  </li>
  <li><strong>Create</strong> the .env file<br>
    <code>cp .env.example .env</code>
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
  <li><strong>Enjoy</strong><br>
  </li>
</ol>
<hr><br>
<h3>Release</h3>
<ol>
  <li><strong>Run</strong> this command<br>
    <code>cargo watch -x "run --release"</code><small>(if cargo-watch installed)</small> <br>
    <code>cargo run --release</code>
  </li>
</ol>