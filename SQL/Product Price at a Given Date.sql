-- Write your PostgreSQL query statement below
WITH last_p AS (
    SELECT DISTINCT ON (product_id)
        product_id,
        LAST_VALUE(new_price) OVER(PARTITION BY product_id ORDER BY change_date DESC) as price
    FROM
        Products
    WHERE
        change_date <= '2019-08-16'
)

SELECT DISTINCT ON(product_id)
    product_id,
    COALESCE(last_p.price, 10) AS price
FROM
    Products
LEFT JOIN
    last_p USING(product_id)