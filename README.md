# FIN CLI

A cli tool which categorizes my transactions based on their description using substring matching and regex.

The transactions are stored in a postgres database in which the schema is found at `schema.sql`

## The gist of it.

The process pretty much goes like this. 

- There are categories, which you create via the database
- There are "rules", which you specify and input via the database. 
Each rule specifies a specific regex or substring match condition (indicated by the `rule_type` column, see [below](#managing-rules-and-categories)) which would map a transaction's description
to a category
- The cli pulls the transactions from the database and processes them with the rules and updates them accordingly

## Setup

- Install via `cargo install --path ./fin_cli`
- Setup the env variable `FIN_DATABASE_URL`
- Create the database via the schema `psql $FIN_DATABASE_URL -f ./schema.sql`

## Available functionality

- Add transactions via csv format using `fin_cli add`, use `fin_cli add --help`, for more info
    - Currently supporting ING csv exports and simple csv exports from bendigo online banking
- Classify all the transactions using `fin_cli classify-all`
- Classify all the transactions with no categories set using `fin_cli classify-uncategorised`

## Managing rules and categories

The cli tool doesn't support this, I personally use [Noco Db](https://github.com/nocodb/nocodb) to manage entries.
But you can use any database client, or just raw sql.

## Rule types

The current rule types must be one of
- "REGEX"
- "STRING_MATCH"

These are specified via the `rule_type` column in the `rule` table.
