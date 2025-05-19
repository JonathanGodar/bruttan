-- Add migration script here
CREATE TYPE swish_payment_request_status AS ENUM ('initialization_failed', 'pending', 'paid', 'declined', 'cancelled', 'timedout');

CREATE TABLE IF NOT EXISTS swish_payment_requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  amount INTEGER NOT NULL,
  token TEXT,
  swish_api_response TEXT,
  status swish_payment_request_status NOT NULL DEFAULT 'pending'
)
