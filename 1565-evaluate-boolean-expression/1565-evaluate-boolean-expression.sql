SELECT e.*,
    CASE WHEN operator = '>' THEN IF(v1.value > v2.value, 'true', 'false')
        WHEN operator = '<' THEN IF(v1.value < v2.value, 'true', 'false')
        ELSE IF(v1.value = v2.value, 'true', 'false') 
    END value
FROM Expressions e
INNER JOIN Variables v1 ON e.left_operand = v1.name
INNER JOIN Variables v2 ON e.right_operand = v2.name