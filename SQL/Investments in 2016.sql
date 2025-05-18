-- Write your PostgreSQL query statement below
WITH cte AS (
    SELECT 
        pid,
        tiv_2015,
        tiv_2016,
        COUNT(CONCAT(lat, lon)) OVER (PARTITION BY CONCAT(lat, lon)) AS cnt1,
        COUNT(tiv_2016) OVER (PARTITION BY tiv_2015) as cnt2
    FROM Insurance 
)

SELECT
    ROUND(SUM(tiv_2016)::decimal, 2) as tiv_2016
FROM
    cte
WHERE
    cnt1 = 1 AND cnt2 != 1
