-- Write your PostgreSQL query statement below
SELECT
    user_id,
    COALESCE(ROUND(SUM(CASE WHEN action = 'confirmed' THEN 1 ELSE 0 END)::decimal / COUNT(*)::decimal, 2), 0) as confirmation_rate
FROM
    Signups
LEFT JOIN
    Confirmations USING(user_id)
GROUP BY
    user_id