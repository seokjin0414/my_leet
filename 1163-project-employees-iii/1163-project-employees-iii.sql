WITH ranked AS (
    SELECT p.project_id, p.employee_id,
        RANK() OVER(PARTITION BY p.project_id ORDER BY e.experience_years DESC) num
    FROM Project p
    INNER JOIN Employee e USING(employee_id)
)
SELECT project_id, employee_id
FROM ranked
WHERE num = 1