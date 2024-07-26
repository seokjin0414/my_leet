SELECT (
    SELECT DISTINCT Salary 
    FROM Employee 
    ORDER BY salary DESC
    LIMIT 1 offset 1) SecondHighestSalary;