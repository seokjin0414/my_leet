SELECT p.player_id, p.player_name, SUM(p.player_id = c.wimbledon) + SUM(p.player_id = c.fr_open) + SUM(p.player_id = c.us_open) + SUM(p.player_id = c.au_open) grand_slams_count
FROM Players p
INNER JOIN Championships c ON p.player_id = c.wimbledon OR p.player_id = c.fr_open OR p.player_id = c.us_open OR p.player_id = c.au_open
GROUP BY 1; 