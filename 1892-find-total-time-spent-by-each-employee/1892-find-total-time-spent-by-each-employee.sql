SELECT event_day day, emp_id, SUM(out_time - in_time) total_time
FROM Employees
GROUP BY 1, 2