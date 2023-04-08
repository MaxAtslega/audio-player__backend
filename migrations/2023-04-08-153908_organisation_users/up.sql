-- Your SQL goes here

CREATE TABLE organisation_users (
  organisation_id INT NOT NULL,
  user_id INT NOT NULL,
  CONSTRAINT fk_organisation_id
    FOREIGN KEY (organisation_id)
    REFERENCES organisation_organisations(id),
  CONSTRAINT fk_user_id
    FOREIGN KEY (user_id)
    REFERENCES user_users(id),
  PRIMARY KEY (organisation_id, user_id)
);
