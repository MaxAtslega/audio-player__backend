-- Your SQL goes here

CREATE TABLE audio_audios (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  file VARCHAR(255) NOT NULL,
  category VARCHAR(255) NOT NULL,
  organisation_id INT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  CONSTRAINT fk_organisation_id
    FOREIGN KEY (organisation_id)
    REFERENCES organisation_organisations(id)
);
