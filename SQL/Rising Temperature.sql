-- Write your PostgreSQL query statement below
WITH last_row AS (
    SELECT 
        *
        , LAG(recordDate) OVER (ORDER BY recordDate) as last_date
        , LAG(temperature) OVER (ORDER BY recordDate) as last_temp
    FROM
        Weather
)

SELECT
    id
FROM 
    last_row
WHERE
    last_date = recordDate - 1 AND temperature > last_temp

# Write your MySQL query statement below
-- Create a CTE to calculate the previous day's temperature and date
WITH q1 AS (
  SELECT 
      *,
      LAG(temperature) OVER (ORDER BY recordDate) AS previous_day_temperature,
      LAG(recordDate) OVER (ORDER BY recordDate) AS previous_Date
  FROM Weather
)

-- Select the IDs where the temperature is higher than the previous day
-- and the previous day exists (difference is exactly 1 day)
SELECT id
FROM q1
WHERE temperature > previous_day_temperature
AND DATEDIFF(recordDate, previous_Date) = 1;