rm recipes.sqlite                                             
sqlite3 recipes.sqlite < api/recipes_db/schema.sql                         
sqlite3 recipes.sqlite < api/recipes_db/sample_data.sql 