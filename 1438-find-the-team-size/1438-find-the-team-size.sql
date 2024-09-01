SELECT employee_id, COUNT(*) OVER(PARTITION BY team_id) team_size
FROM employee