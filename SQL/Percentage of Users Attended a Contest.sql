-- Write your PostgreSQL query statement below
SELECT
    contest_id,
    ROUND(COUNT(contest_id)::decimal / (SELECT COUNT(user_id) FROM Users), 4) * 100 as "percentage"
FROM
    Register
GROUP BY
    contest_id
ORDER BY
    "percentage" DESC, contest_id ASC