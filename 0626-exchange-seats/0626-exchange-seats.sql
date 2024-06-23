SELECT
ROW_NUMBER() OVER(
    ORDER BY 
    CASE 
    WHEN MOD(id, 2) = 0 THEN id - 1 
    ELSE id + 1 
    END
    ) id,
student
FROM seat;