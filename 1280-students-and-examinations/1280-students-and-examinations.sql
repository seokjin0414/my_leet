
SELECT st.student_id, st.student_name, su.subject_name, COALESCE(e.attended_exams, 0) attended_exams
FROM Students st
CROSS JOIN Subjects su
LEFT JOIN (
    SELECT student_id, subject_name, COUNT(*) attended_exams
    FROM Examinations
    GROUP BY student_id, subject_name
) e USING (student_id, subject_name)
ORDER BY st.student_id, su.subject_name;