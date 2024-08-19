SELECT e.*,
    IF(CASE WHEN operator = '>' THEN v1.value > v2.value
        WHEN operator = '<' THEN v1.value < v2.value
        ELSE v1.value = v2.value
    END, "true", "false") value
FROM Expressions e
INNER JOIN Variables v1 ON e.left_operand = v1.name
INNER JOIN Variables v2 ON e.right_operand = v2.name