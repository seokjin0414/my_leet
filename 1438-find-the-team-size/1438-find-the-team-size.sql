WITH size AS (
    SELECT team_id, COUNT(*) cnt
    FROM Employee
    GROUP BY 1
)
SELECT e.employee_id, s.cnt team_size
FROM Employee e
INNER JOIN size s USING(team_id)
