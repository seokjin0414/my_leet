SELECT t.team_id, t.team_name,
    SUM(CASE WHEN t.team_id = m.host_team AND m.host_goals > m.guest_goals THEN 3
        WHEN t.team_id = m.guest_team AND m.guest_goals > m.host_goals THEN 3
        WHEN t.team_id = m.host_team AND m.host_goals = m.guest_goals THEN 1
        WHEN t.team_id = m.guest_team AND m.guest_goals = m.host_goals THEN 1 ELSE 0 END) num_points
FROM Teams t
LEFT JOIN Matches m ON t.team_id = m.host_team OR t.team_id = m.guest_team
GROUP BY 1
ORDER BY 3 DESC, 1;