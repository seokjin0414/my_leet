WITH RECURSIVE ids AS (
SELECT task_id, subtasks_count, 1 subtask_id 
FROM Tasks
UNION ALL 
SELECT task_id, subtasks_count, subtask_id + 1 subtask_id 
FROM ids 
WHERE subtask_id < (SELECT MAX(subtasks_count) FROM Tasks)
)
SELECT i.task_id, i.subtask_id 
FROM ids i
LEFT JOIN Executed e ON i.task_id = e.task_id AND i.subtask_id = e.subtask_id 
WHERE e.subtask_id IS NULL 
AND i.subtasks_count >= i.subtask_id 