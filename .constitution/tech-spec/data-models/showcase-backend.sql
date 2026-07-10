-- NORMATIVE SEED — showcase Backend (ADR-008).
-- PostgreSQL 17 DDL for backends/showcase; served by PostgREST 13.
-- Doubles as: catalog-frontend introspection fixture, OpenAPI-frontend source
-- (PostgREST-generated description), Provider conformance target, and the
-- dataset behind the public showcase (CAP-1001).
-- Relational on purpose: exercises belongs_to/has_many, enums, nullability,
-- server-owned columns, and volume (NFC-01 seeding is a separate script).

create schema if not exists showcase;

create type showcase.user_status as enum ('active', 'suspended', 'invited');
create type showcase.ticket_priority as enum ('low', 'normal', 'high', 'urgent');

create table showcase.organizations (
    id          uuid primary key default gen_random_uuid(),   -- server-owned
    name        text not null,
    plan        text not null default 'free',
    created_at  timestamptz not null default now()            -- server-owned
);

create table showcase.users (
    id           uuid primary key default gen_random_uuid(),  -- server-owned
    org_id       uuid not null references showcase.organizations(id),
    name         text not null,
    email        text not null unique,
    status       showcase.user_status not null default 'invited',
    last_seen_at timestamptz,                                 -- nullable
    created_at   timestamptz not null default now()           -- server-owned
);

create table showcase.tickets (
    id          uuid primary key default gen_random_uuid(),   -- server-owned
    org_id      uuid not null references showcase.organizations(id),
    assignee_id uuid references showcase.users(id),           -- nullable FK
    title       text not null,
    body        text,
    priority    showcase.ticket_priority not null default 'normal',
    open        boolean not null default true,
    created_at  timestamptz not null default now(),           -- server-owned
    updated_at  timestamptz not null default now()            -- server-owned; touch-trigger
);

-- Windowed-list workloads (filter/sort P0 paths)
create index tickets_org_created_idx  on showcase.tickets (org_id, created_at desc);
create index tickets_assignee_idx     on showcase.tickets (assignee_id) where assignee_id is not null;
create index users_org_idx            on showcase.users (org_id);
create index users_status_idx         on showcase.users (status);

-- PostgREST roles: anonymous read/write inside the showcase schema only.
-- (No RLS in the P0 showcase — auth is P1; the schema boundary is the demo scope.)
create role showcase_anon nologin;
grant usage on schema showcase to showcase_anon;
grant select, insert, update, delete on all tables in schema showcase to showcase_anon;

-- updated_at touch trigger (server-owned column semantics for drift/demo purposes)
create or replace function showcase.touch_updated_at() returns trigger as $$
begin new.updated_at = now(); return new; end;
$$ language plpgsql;

create trigger tickets_touch before update on showcase.tickets
    for each row execute function showcase.touch_updated_at();
