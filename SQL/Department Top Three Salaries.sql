-- Write your PostgreSQL query statement below
WITH cte as (SELECT
    d.name as Department,
    e.name as Employee,
    e.salary as Salary,
    DENSE_RANK() OVER(PARTITION BY d.id ORDER BY salary DESC) AS rnk
FROM
    Employee AS e
JOIN
    Department AS d ON d.id = e.departmentId)

SELECT
    Department,
    Employee,
    Salary
FROM
    cte
WHERE
    rnk <= 3