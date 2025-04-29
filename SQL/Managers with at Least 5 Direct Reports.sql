-- Write your PostgreSQL query statement below
SELECT
    e1.name
FROM
    Employee as e1
JOIN Employee as e2 ON e1.id = e2.managerID
GROUP BY
    e1.id, e1.name
HAVING
    COUNT(*) > 4