-- Write your PostgreSQL query statement below
SELECT
    DISTINCT(num) as "ConsecutiveNums"
FROM
    (
    SELECT
        LEAD(num) OVER(ORDER BY id) as lead,
        num,
        LAG(num) OVER(ORDER BY id) as lag
    FROM
        LOGS
    )
WHERE
    lead = num AND lag = num