-- Write your PostgreSQL query statement below
(SELECT
    name as results
FROM
    MovieRating
JOIN
    Users USING(user_id)
ORDER BY
    COUNT(rating) OVER(PARTITION BY user_id) DESC, name ASC
LIMIT 1)

UNION ALL

(SELECT
    title as results
FROM
    MovieRating
JOIN
    Movies USING(movie_id)
WHERE
    created_at BETWEEN '2020-02-01' AND '2020-02-29'
ORDER BY
    AVG(rating) OVER(PARTITION BY movie_id) DESC, title ASC
LIMIT 1)