-- Write your PostgreSQL query statement below
WITH CTE AS (
    SELECT DISTINCT
        player_id,
        event_date,
        FIRST_VALUE(event_date) OVER(PARTITION BY player_id ORDER BY event_date ASC) + 1 as fd
    FROM
        Activity
)

SELECT
    ROUND(SUM(CASE WHEN event_date = fd THEN 1 ELSE 0 END) / COUNT(DISTINCT player_id)::decimal, 2) AS fraction
FROM
    CTE