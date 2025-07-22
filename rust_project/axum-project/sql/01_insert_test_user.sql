BEGIN;


INSERT INTO users (username, password)
VALUES ('test', 'test');


SELECT *
FROM users;


COMMIT;