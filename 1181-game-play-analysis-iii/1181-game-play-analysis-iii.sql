SELECT a.player_id, a.event_date, SUM(aa.games_played) games_played_so_far
FROM Activity a
INNER JOIN Activity aa ON a.player_id = aa.player_id AND a.event_date >= aa.event_date
GROUP BY 1, 2