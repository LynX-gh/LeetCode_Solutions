-- Write your PostgreSQL query statement below
SELECT
    euni.unique_id
    , e.name
FROM
    Employees as e
LEFT JOIN
    EmployeeUNI as euni
ON
    e.id = euni.id