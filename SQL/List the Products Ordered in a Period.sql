-- Write your PostgreSQL query statement below
SELECT 
    p.product_name, 
    SUM(o.unit) AS unit
FROM 
    Orders o 
NATURAL JOIN 
    Products p
WHERE 
    o.order_date BETWEEN '2020-02-01' AND '2020-02-29'
GROUP BY 
    p.product_id, p.product_name
HAVING 
    SUM(o.unit) >= 100;