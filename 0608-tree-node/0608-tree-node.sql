SELECT t1.id, 
    CASE WHEN t1.p_id IS NULL THEN 'Root'
    WHEN EXISTS(SELECT * FROM Tree t2 WHERE t2.p_id = t1.id)
    THEN 'Inner'
    ELSE 'Leaf' 
    END type
FROM Tree t1
