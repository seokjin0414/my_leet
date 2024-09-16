WITH data AS(
    SELECT s.student_id, s.student_name, e.exam_id, e.score, 
    MAX(score) OVER(PARTITION BY exam_id ) max_score,
    MIN(score) OVER(PARTITION BY exam_id ) min_score
    FROM student s 
    INNER JOIN exam e USING(student_id)
),
not_quiet AS (
    SELECT student_id, student_name
    FROM data 
    WHERE score = max_score 
    OR score = min_score
    GROUP BY 1
)
SELECT student_id, student_name
FROM data 
WHERE student_id NOT IN (SELECT student_id FROM not_quiet)
GROUP BY 1
ORDER BY 1