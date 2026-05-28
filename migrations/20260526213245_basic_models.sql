CREATE TABLE IF NOT EXISTS chapter_event (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  total_seats INTEGER CHECK (total_seats > 0),
  max_tickets_per_payment INTEGER,
  sales_stop_at TIMESTAMPTZ NOT NULL,
  reservation_duration_seconds INTEGER,
  event_at TIMESTAMPTZ NOT NULL,
  door_open_before TIMESTAMPTZ,

  fcfs BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS ticket_type(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  price INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  is_visible BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS jt_chapter_event_ticket_type(
  chapter_event_id UUID NOT NULL REFERENCES chapter_event(id),
  ticket_type_id UUID NOT NULL REFERENCES ticket_type(id)
);


CREATE TABLE IF NOT EXISTS ticket (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  external_id TEXT UNIQUE NOT NULL,
  time_created TIMESTAMPTZ NOT NULL DEFAULT now(),
  ticket_type_id UUID NOT NULL REFERENCES ticket_type(id),
  chapter_event_id UUID NOT NULL REFERENCES chapter_event(id),
  times_used INTEGER NOT NULL DEFAULT 0);




