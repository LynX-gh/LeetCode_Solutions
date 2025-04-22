-- Write your PostgreSQL query statement below
SELECT
    v.customer_id
    , COUNT(visit_id) as count_no_trans
FROM
    Visits as v
LEFT JOIN
    Transactions as t USING (visit_id)
WHERE
    transaction_id is NULL
GROUP BY
    v.customer_id