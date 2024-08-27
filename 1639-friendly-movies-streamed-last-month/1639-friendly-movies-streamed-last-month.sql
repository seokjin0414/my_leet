SELECT DISTINCT c.title
FROM TVProgram t
INNER JOIN Content c ON t.content_id = c.content_id AND c.Kids_content = 'Y' AND c.content_type = 'movies'
WHERE DATE_FORMAT(t.program_date, '%Y-%m') = '2020-06'