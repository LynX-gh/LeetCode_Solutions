# Write your MySQL query statement below
SELECT
    e.employee_id
FROM
    Employees AS e
WHERE
    NOT EXISTS (SELECT employee_id from Employees where e.manager_id = employee_id) AND e.manager_id IS NOT NULL AND e.salary < 30000
ORDER BY
    e.employee_id