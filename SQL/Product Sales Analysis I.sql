-- Write your PostgreSQL query statement below
SELECT
    p.product_name
    , s.year
    , s.price
FROM
    Sales as s
JOIN
    Product as p USING (product_id)