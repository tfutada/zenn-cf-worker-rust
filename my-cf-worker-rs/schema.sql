DROP TABLE IF EXISTS Customers;
CREATE TABLE IF NOT EXISTS Customers (customer_id INTEGER PRIMARY KEY, company_name TEXT, contact_name TEXT);
INSERT INTO Customers (CustomerID, company_name, contact_name) VALUES (1, 'Alfreds Futterkiste', 'Maria Anders'), (4, 'Around the Horn', 'Thomas Hardy'), (11, 'Bs Beverages', 'Victoria Ashworth'), (13, 'Bs Beverages', 'Random Name');