with cte as(select customer_id,
group_concat(product_name) as products 
from orders group by customer_id
having find_in_set('C',products) = 0 and 
find_in_set('A',products) > 0 and 
find_in_set('B',products) > 0)
select cte.customer_id,c.customer_name from Customers as c join cte on cte.customer_id = c.customer_id