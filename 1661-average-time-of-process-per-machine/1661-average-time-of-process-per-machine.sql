SELECT a.machine_id, ROUND(AVG(b.timestamp - a.timestamp), 3) processing_time
FROM activity a
INNER JOIN activity b ON b.machine_id = a.machine_id AND b.process_id = a.process_id AND a.activity_type = 'start' AND b.activity_type = 'end'
GROUP BY 1;
