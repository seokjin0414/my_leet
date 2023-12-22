SELECT p.project_id, ROUND(AVG(e.experience_years), 2) average_years
FROM Project p
INNER JOIN Employee e ON e.employee_id = p.employee_id
GROUP BY 1