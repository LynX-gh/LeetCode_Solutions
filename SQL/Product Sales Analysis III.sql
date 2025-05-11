-- Write your PostgreSQL query statement below
WITH CTE_FirstYear AS (
    SELECT
        product_id,
        year,
        FIRST_VALUE(year) OVER (PARTITION BY product_id ORDER BY year ASC) as first_y,        
        quantity,
        price
    FROM
        Sales
    JOIN
        Product USING (product_id)
)

SELECT
    product_id,
    year as first_year,
    quantity,
    price
FROM
    CTE_FirstYear
WHERE
    year = first_y
 