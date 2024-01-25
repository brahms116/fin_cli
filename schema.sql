create table if not exists category (
  id text not null default gen_random_uuid() primary key, 
  name text not null,
  description text not null,
  date_created timestamp not null default now()
);

create table if not exists rule (
  id text not null default gen_random_uuid() primary key,
  name text not null,
  description text not null,
  category text not null references category(id) on delete cascade on update cascade,
  rule_type text not null,
  pattern text not null,
  date_created timestamp not null default now()
);

create table if not exists transaction (
  id text not null primary key,
  date date not null,
  description text not null,
  amount_cents integer not null,
  category text references category(id) on delete set null on update cascade,
  date_created timestamp not null default now()
);

create index if not exists transaction_date_idx on transaction(date);
create index if not exists transaction_category_idx on transaction(category);
