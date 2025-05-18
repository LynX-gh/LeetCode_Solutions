-- Write your PostgreSQL query statement below
SELECT
    COALESCE(r.user, a.user) as id,
    COALESCE(r.cnt, 0) + COALESCE(a.cnt, 0) as num
FROM
    (SELECT requester_id as user, COUNT(requester_id) as cnt FROM RequestAccepted GROUP BY requester_id) AS r
FULL JOIN
    (SELECT accepter_id as user, COUNT(accepter_id) as cnt FROM RequestAccepted GROUP BY accepter_id) AS a
ON r.user = a.user
ORDER BY
    num DESC
LIMIT 1