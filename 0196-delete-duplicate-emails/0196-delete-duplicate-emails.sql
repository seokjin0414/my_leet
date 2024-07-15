DELETE p
FROM person p, person pe
WHERE p.email = pe.email
AND p.id > pe.id;