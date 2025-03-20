# Write your MySQL query statement below
SELECT c.name as "Customers"
FROM Customers as c
LEFT JOIN Orders as o ON c.id = o.customerID
WHERE o.id is NULL