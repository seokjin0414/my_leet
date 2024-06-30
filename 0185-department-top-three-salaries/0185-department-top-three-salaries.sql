WITH base AS (
    SELECT d.name Department, e.name Employee, e.salary Salary, 
        DENSE_RANK() OVER(PARTITION BY d.id ORDER BY e.salary DESC) d_num
    FROM Employee e
    INNER JOIN Department d ON e.departmentId = d.id  
)
SELECT Department, Employee, Salary
FROM base
WHERE d_num <= 3
ORDER BY 1, 3 DESC;