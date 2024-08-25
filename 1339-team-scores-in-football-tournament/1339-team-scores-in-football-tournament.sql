SELECT t.team_id, t.team_name,
    SUM(CASE WHEN team_id=host_team AND host_goals>guest_goals THEN 3
        WHEN team_id=guest_team AND guest_goals>host_goals THEN 3
        WHEN team_id=host_team AND host_goals=guest_goals THEN 1
        WHEN team_id=guest_team AND guest_goals=host_goals THEN 1 ELSE 0 END) num_points
FROM Teams t
LEFT JOIN Matches m ON t.team_id = m.host_team OR t.team_id = m.guest_team
GROUP BY 1
ORDER BY 3 DESC, 1;