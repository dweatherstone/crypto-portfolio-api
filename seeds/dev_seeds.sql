-- Clean up existing seed data if re-running (optional, but keeps seeds idempotent)
-- Uses fixed UUIDs so you can hardcode them in curl/test scripts

-- Insert mock users
INSERT INTO users (id, email, password_hash)
VALUES 
  ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'alice@example.com', '$2b$12$e83/dummyhashalice'),
  ('b1ffcd00-0d1c-5fa9-cc7e-7cc0ce491b22', 'bob@example.com', '$2b$12$e83/dummyhashbob')
ON CONFLICT (email) DO NOTHING;

-- Insert portfolios
INSERT INTO portfolios (id, user_id, name)
VALUES 
  ('c2000000-0000-0000-0000-000000000001', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'Main Crypto Vault'),
  ('c2000000-0000-0000-0000-000000000002', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'DeFi Staking'),
  ('c2000000-0000-0000-0000-000000000003', 'b1ffcd00-0d1c-5fa9-cc7e-7cc0ce491b22', 'Bob Cold Storage')
ON CONFLICT DO NOTHING;

-- Insert holdings
INSERT INTO holdings (portfolio_id, symbol, amount)
VALUES 
  ('c2000000-0000-0000-0000-000000000001', 'BTC', 1.450000000000),
  ('c2000000-0000-0000-0000-000000000001', 'ETH', 12.800000000000),
  ('c2000000-0000-0000-0000-000000000002', 'SOL', 150.000000000000),
  ('c2000000-0000-0000-0000-000000000003', 'BTC', 10.000000000000)
ON CONFLICT DO NOTHING;

-- Insert price alerts
INSERT INTO price_alerts (user_id, symbol, target_price, condition)
VALUES
  ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'BTC', 70000.00, 'ABOVE'),
  ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'ETH', 3000.00, 'BELOW')
ON CONFLICT DO NOTHING;