CREATE TABLE IF NOT EXISTS hero (
	id SERIAL NOT NULL,
	"name" character varying(255) NOT NULL,
	"identity" character varying(255) NOT NULL,
	hometown character varying(255) NOT NULL,
	age INT NOT NULL,
	created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP,
	PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('hero');
