-- Add migration script here

CREATE TABLE stock_asset_allocation (
    date TIMESTAMP NOT NULL,
    percentage DECIMAL NOT NULL,
);

CREATE TABLE sp_500_monthly (
    date TIMESTAMP NOT NULL,
    close DECIMAL NOT NULL,
    dividend DECIMAL NOT NULL,
    earnings DECIMAL NOT NULL,
    cpi DECIMAL NOT NULL,
    gs10 DECIMAL NOT NULL,
    pe10 DECIMAL NOT NULL,
    adjusted_close DECIMAL NOT NULL,
);

CREATE TABLE sp_500_daily (
    date TIMESTAMP NOT NULL,
    value DECIMAL NOT NULL,
);

CREATE TABLE unemployment_rate (
    date TIMESTAMP NOT NULL,
    percentage DECIMAL NOT NULL,
);

CREATE TABLE recessions (
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
);