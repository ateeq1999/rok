DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS auth_access_tokens CASCADE;

CREATE TABLE users (
  id VARCHAR(26) PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  full_name VARCHAR(100),
  phone VARCHAR(20),
  role VARCHAR(50) DEFAULT 'user',
  email_verified BOOLEAN DEFAULT false,
  phone_verified BOOLEAN DEFAULT false,
  marketing_consent BOOLEAN DEFAULT false,
  points INTEGER DEFAULT 0,
  email_verified_at TIMESTAMP,
  phone_verified_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE auth_access_tokens (
  id SERIAL PRIMARY KEY,
  tokenable_id VARCHAR(26) NOT NULL,
  tokenable_type VARCHAR(255) NOT NULL,
  name VARCHAR(255),
  type VARCHAR(255) NOT NULL,
  abilities TEXT,
  expires_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE events (
  id VARCHAR(26) PRIMARY KEY,
  title VARCHAR(200) NOT NULL,
  description TEXT,
  cover_image_url TEXT,
  location VARCHAR(300) NOT NULL,
  city VARCHAR(100) NOT NULL,
  country VARCHAR(100) DEFAULT 'Tanzania',
  address TEXT,
  latitude DECIMAL(10, 7),
  longitude DECIMAL(10, 7),
  status VARCHAR(50) DEFAULT 'draft',
  category VARCHAR(50),
  capacity INTEGER,
  rsvp_count INTEGER DEFAULT 0,
  starts_at TIMESTAMP NOT NULL,
  ends_at TIMESTAMP,
  sales_opens_at TIMESTAMP,
  sales_closes_at TIMESTAMP,
  is_featured BOOLEAN DEFAULT false,
  is_public BOOLEAN DEFAULT true,
  organizer_id VARCHAR(26),
  published_at TIMESTAMP,
  deleted_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE ticket_types (
  id VARCHAR(26) PRIMARY KEY,
  event_id VARCHAR(26) NOT NULL,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  price INTEGER DEFAULT 0,
  currency VARCHAR(10) DEFAULT 'TZS',
  quantity_total INTEGER NOT NULL,
  quantity_sold INTEGER DEFAULT 0,
  status VARCHAR(50) DEFAULT 'active',
  min_per_order INTEGER DEFAULT 1,
  max_per_order INTEGER DEFAULT 10,
  perks TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);