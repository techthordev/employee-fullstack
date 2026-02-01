-- employee-directory.sql
-- Run as postgres superuser during container startup

-- Clean up user if exists
DROP USER IF EXISTS springstudent;

-- Create application user
CREATE USER springstudent WITH PASSWORD 'springstudent';

-- Grant full access to the existing database (created by POSTGRES_DB)
GRANT ALL PRIVILEGES ON DATABASE employee_directory TO springstudent;

\connect employee_directory

-- Create table
CREATE TABLE IF NOT EXISTS employee (
                                        id         SERIAL PRIMARY KEY,
                                        first_name VARCHAR(45) DEFAULT NULL,
    last_name  VARCHAR(45) DEFAULT NULL,
    email      VARCHAR(45) DEFAULT NULL
    );

-- Insert sample data
INSERT INTO employee (first_name, last_name, email) VALUES
                                                        ('Leslie',  'Andrews',    'leslie@luv2code.com'),
                                                        ('Emma',    'Baumgarten', 'emma@luv2code.com'),
                                                        ('Avani',   'Gupta',      'avani@luv2code.com'),
                                                        ('Yuri',    'Petrov',     'yuri@luv2code.com'),
                                                        ('Juan',    'Vega',       'juan@luv2code.com');

-- Make sure sequence is set correctly
SELECT setval('employee_id_seq', (SELECT MAX(id) FROM employee));

-- Grant rights on existing objects
GRANT ALL PRIVILEGES ON TABLE employee TO springstudent;
GRANT ALL PRIVILEGES ON SEQUENCE employee_id_seq TO springstudent;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO springstudent;
ALTER DEFAULT PRIVILEGES ON SEQUENCES TO springstudent;
