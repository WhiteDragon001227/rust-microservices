CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  "uuid" uuid NOT NULL DEFAULT gen_random_uuid(),
  email character varying NOT NULL,
  password_digest character varying DEFAULT ''::character varying NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX index_users_uuid ON users USING btree(uuid);
SELECT diesel_manage_updated_at('users');
