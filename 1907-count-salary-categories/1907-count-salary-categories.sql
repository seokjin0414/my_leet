WITH vi AS(
SELECT * FROM Accounts
)
SELECT "Low Salary" category, SUM(income < 20000) accounts_count
FROM vi
UNION
SELECT "Average Salary" category, SUM(income BETWEEN 20000 AND 50000) accounts_count
FROM vi
UNION
SELECT "High Salary" category, SUM(income > 50000) accounts_count
FROM vi;