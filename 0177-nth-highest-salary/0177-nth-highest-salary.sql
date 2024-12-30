CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  RETURN (
    WITH cal AS (
        SELECT *, DENSE_RANK() OVER(ORDER BY salary DESC) num
        FROM Employee
    )
    SELECT MAX(salary)
    FROM cal
    WHERE num = N
  );
END