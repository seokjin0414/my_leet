
SELECT s.student_id, s.student_name
FROM Student s
JOIN (
    SELECT a.student_id, COUNT(a.exam_id) AS total_exam,
        SUM(
            CASE
                WHEN a.score > min_score AND a.score < max_score THEN 1
                ELSE 0
            END
        ) AS quite_exam
    FROM Exam a
    JOIN (
        SELECT exam_id, MIN(score) AS min_score, MAX(score) AS max_score
        FROM Exam
        GROUP BY exam_id
    ) b
    ON a.exam_id = b.exam_id
    GROUP BY a.student_id
) c
ON s.student_id = c.student_id
WHERE c.total_exam = c.quite_exam