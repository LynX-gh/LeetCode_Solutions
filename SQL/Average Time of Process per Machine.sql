# Write your MySQL query statement below
SELECT
    m1.machine_id as machine_id
    ,ROUND((SUM(m2.timestamp) - SUM(m1.timestamp)) / COUNT(m1.machine_id), 3) as processing_time
FROM
    Activity as m1
JOIN
    Activity as m2
ON
    m1.machine_id = m2.machine_id AND m1.process_id = m2.process_id AND m1.activity_type = 'start' AND m2.activity_type = 'end'
GROUP BY
    m1.machine_id