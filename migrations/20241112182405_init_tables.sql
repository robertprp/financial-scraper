-- Add migration script here

CREATE TABLE IF NOT EXISTS stock_asset_allocation (
    "date" TIMESTAMPTZ NOT NULL,
    percentage DECIMAL NOT NULL
);

CREATE TABLE IF NOT EXISTS sp_500_monthly (
    "date" TIMESTAMPTZ NOT NULL,
    "close" DECIMAL,
    dividend DECIMAL,
    earnings DECIMAL,
    cpi DECIMAL,
    gs10 DECIMAL,
    pe10 DECIMAL,
    adjusted_close DECIMAL
);

CREATE TABLE IF NOT EXISTS sp_500_daily (
    "date" TIMESTAMPTZ NOT NULL,
    "value" DECIMAL NOT NULL
);

CREATE TABLE IF NOT EXISTS unemployment_rate (
    "date" TIMESTAMPTZ NOT NULL,
    percentage DECIMAL NOT NULL
);

CREATE TABLE  IF NOT EXISTS recessions (
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL
);