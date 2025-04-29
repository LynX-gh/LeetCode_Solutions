-- Write your PostgreSQL query statement below
SELECT
    s.student_id
    ,s.student_name
    ,sub.subject_name
    ,COUNT(e.student_id) AS attended_exams
FROM
    Students AS s
CROSS JOIN
    Subjects AS sub
LEFT JOIN
    Examinations AS e
ON
    e.student_id = s.student_id AND sub.subject_name = e.subject_name
GROUP BY
    s.student_id, s.student_name, sub.subject_name
ORDER BY
    s.student_id, s.student_name, sub.subject_name