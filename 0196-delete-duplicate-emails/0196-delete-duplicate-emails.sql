DELETE p
FROM person p
INNER JOIN person pe ON p.email = pe.email AND p.id > pe.id