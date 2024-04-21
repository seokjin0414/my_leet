WITH counted_one AS(
    SELECT employee_id
    FROM Employee
    GROUP BY employee_id
    HAVING COUNT(employee_id) = 1
)
SELECT DISTINCT employee_id, department_id
FROM Employee
WHERE employee_id IN (SELECT * FROM counted_one)
OR primary_flag = 'Y'
ORDER BY 1;