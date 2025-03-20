# Write your MySQL query statement below
SELECT p.email
FROM Person as p
GROUP BY p.email
HAVING COUNT(p.email) > 1