SELECT u.name, SUM(t.amount) balance
FROM Users u
INNER JOIN Transactions t USING (account)
GROUP BY 1
HAVING SUM(t.amount) > 10000
