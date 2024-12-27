WITH data AS (
    SELECT employee_id, experience,
        SUM(salary) OVER(PARTITION BY experience ORDER BY salary) salary_sum
    FROM Candidates
),
senior AS (
    SELECT 'Senior' experience,
        COUNT(employee_id) accepted_candidates,
        IFNULL(MAX(salary_sum), 0) cost
    FROM data
    WHERE experience = 'Senior'
    AND salary_sum <= 70000
),
junior AS (
    SELECT 'Junior' experience, COUNT(employee_id) accepted_candidates
    FROM data
    WHERE experience = 'Junior'
    AND salary_sum <= 70000 - (SELECT cost FROM senior)
)
SELECT experience, accepted_candidates
FROM senior
UNION ALL
SELECT experience, accepted_candidates
FROM junior