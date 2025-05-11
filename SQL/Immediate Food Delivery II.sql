-- Write your PostgreSQL query statement below
WITH first_orders AS (
    SELECT DISTINCT ON (customer_id)
        customer_id,
        FIRST_VALUE(order_date) OVER(
            PARTITION BY customer_id
            ORDER BY order_date
        ) as order_date,
        FIRST_VALUE(customer_pref_delivery_date) OVER(
            PARTITION BY customer_id
            ORDER BY order_date
        ) as cpdd
    FROM
        Delivery
)

SELECT
    ROUND(SUM(CASE WHEN order_date = cpdd THEN 1 ELSE 0 END)::decimal / COUNT(*), 4) * 100 as immediate_percentage
FROM
    first_orders