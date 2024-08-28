SELECT seat_id FROM (
  SELECT seat_id, free, 
    LEAD(free,1,FALSE) OVER (ORDER BY seat_id) free_1,
    LAG(free,1,FALSE) OVER (ORDER BY seat_id) free_2
  FROM cinema
) d
WHERE free 
AND (free_1 OR free_2)