WITH ranked AS (
SELECT ROW_NUMBER() OVER(PARTITION BY student_id ORDER BY grade DESC, course_id) num, student_id, course_id, grade
FROM Enrollments    
)  
SELECT student_id, course_id, grade
FROM ranked
WHERE num = 1;