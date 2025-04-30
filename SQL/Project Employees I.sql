-- Write your PostgreSQL query statement below
SELECT
    p.project_id,
    ROUND(SUM(e.experience_yearsdecimal)  COUNT(e.employee_id), 2) AS average_years
FROM
    Project AS p
JOIN
    Employee AS e
ON
    e.employee_id = p.employee_id
GROUP BY
    p.project_id