-- Write your PostgreSQL query statement below
SELECT
    employee_id,
    department_id
FROM
    (
        SELECT
            *,
            COUNT(department_id) OVER (PARTITION BY employee_id) AS DeptCnt
        FROM
            Employee
    ) EmployeePartition
WHERE
    DeptCnt = 1 OR primary_flag = 'Y'