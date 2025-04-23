-- Write your PostgreSQL query statement below
SELECT
    name
    ,bonus
FROM
    Employee
LEFT JOIN
    Bonus USING(empId)
WHERE bonus < 1000 OR bonus IS NULL