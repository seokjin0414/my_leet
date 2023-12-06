SELECT U.unique_id, E.name
FROM Employees E
LEFT JOIN EmployeeUNI U ON U.id = E.id
