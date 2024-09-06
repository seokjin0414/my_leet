SELECT DISTINCT l1.account_id
FROM LogInfo l1
INNER JOIN LogInfo l2 USING(account_id)
WHERE (l1.login BETWEEN l2.login AND l2.logout
AND l1.ip_address != l2.ip_address)
OR (l1.logout BETWEEN l2.login AND l2.logout
AND l1.ip_address != l2.ip_address);
