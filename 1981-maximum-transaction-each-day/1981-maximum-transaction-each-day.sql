WITH max_amount AS (
    SELECT DATE(day) target_date, MAX(amount) amount
    FROM Transactions
    GROUP BY 1
) 
SELECT t.transaction_id
FROM Transactions t
INNER JOIN max_amount m ON DATE(t.day) = m.target_date AND t.amount = m.amount
ORDER BY 1