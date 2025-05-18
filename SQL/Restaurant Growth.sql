-- Write your PostgreSQL query statement below
WITH data AS (SELECT
    visited_on,
    ROUND(SUM(amount) OVER(ORDER BY visited_on RANGE BETWEEN INTERVAL '6 days' PRECEDING AND CURRENT ROw), 2) AS amount,
    ROUND(AVG(amount) OVER(ORDER BY visited_on RANGE BETWEEN INTERVAL '6 days' PRECEDING AND CURRENT ROw), 2) AS average_amount
FROM
    (SELECT visited_on, sum(amount) as amount FROM Customer GROUP BY visited_on) AS Customer
ORDER BY
    visited_on ASC)

SELECT
    *
FROM
    data
WHERE
    visited_on - INTERVAL '6 DAY' IN (SELECT visited_on FROM Customer)