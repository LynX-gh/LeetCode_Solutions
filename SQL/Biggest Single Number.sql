-- Write your PostgreSQL query statement below
With CTE_nums 
as (
    SELECT num
    FROM MyNumbers
    GROUP BY num
    HAVING COUNT(*) = 1
)

SELECT
    MAX(num) as num
FROM
    CTE_nums