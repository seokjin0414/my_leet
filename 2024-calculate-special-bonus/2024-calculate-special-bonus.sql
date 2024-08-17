SELECT employee_id , salary * ( employee_id%2 ) * (name NOT LIKE 'M%') bonus
FROM Employees
ORDER BY 1;