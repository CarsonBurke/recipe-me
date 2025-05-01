-- Sample data for the recipe database

-- Insert users
INSERT INTO "User" (username, email, password) VALUES
    ('john_doe', 'john@example.com', '$2a$10$dSVWRR9x7e0UwklQYGt1UOSLOeJ7q5CHQ1ru1d2hAg3lJcKySkhw.'), -- password: P@ssw0rd123
    ('jane_smith', 'jane@example.com', '$2a$10$5VVuEfVxYAVUCuJuGVGu4uG/P6o3s4djjsQJ9B9Py8h.7S1XmCOZu'), -- password: Jane2024!
    ('chef_mike', 'mike@cheflife.com', '$2a$10$jLnSc3UmvA.3SdJzSOY5U.yZ22WgXiKF7XGX7V5MFadB8gTXVhDVa'), -- password: ChefMike!23
    ('foodie_lisa', 'lisa@foodie.com', '$2a$10$nEwI3PjYhzwbkQs3pHtVXegPSKEwSzpZSRnKBfT26x4xJJtGSV9Xu'), -- password: FoodLover456
    ('cooking_guru', 'guru@cooking.com', '$2a$10$GvYx9ThVmNx.BOxb1E2P1O1c7lxjBmL0RhLTeNtcE9XwPW5agF4E.'); -- password: Guru@Cook789

-- Insert ingredient names
INSERT INTO "IngredientName" (name) VALUES
    ('salt'),
    ('pepper'),
    ('olive oil'),
    ('garlic'),
    ('onion'),
    ('tomato'),
    ('chicken breast'),
    ('beef'),
    ('rice'),
    ('pasta'),
    ('flour'),
    ('sugar'),
    ('butter'),
    ('eggs'),
    ('milk'),
    ('cheese'),
    ('basil'),
    ('oregano'),
    ('cinnamon'),
    ('vanilla extract'),
    ('bell pepper'),
    ('carrot'),
    ('potato'),
    ('lemon'),
    ('ginger'),
    ('soy sauce'),
    ('honey'),
    ('chocolate'),
    ('almonds'),
    ('spinach');

-- Insert recipes
INSERT INTO "Recipe" (name, summary, description, instructions, ingredients, views, ratings, total_rating) VALUES
    ('Classic Spaghetti Bolognese', 
     'Spaghetti with meat sauce', 
     'A rich and hearty Italian pasta dish with a meaty sauce.', 
     'Start by sautéing onions and garlic. Add ground beef and cook until browned. Add tomatoes and simmer for 30 minutes. Serve over cooked pasta.', 
     'Ground beef, Onion, Garlic, Tomatoes, Pasta, Olive oil, Salt, Pepper, Oregano', 
     1205, 42, 189),
    
    ('Chicken Alfredo', 
     'Creamy pasta with grilled chicken and parmesan cheese', 
     'Creamy pasta with grilled chicken and parmesan cheese.', 
     'Cook pasta according to package directions. In a separate pan, sauté chicken until cooked through. Make a cream sauce with butter, cream, and parmesan. Combine all ingredients.', 
     'Chicken breast, Fettuccine pasta, Heavy cream, Butter, Parmesan cheese, Garlic, Salt, Pepper', 
     980, 35, 158),
    
    ('Vegetable Stir Fry', 
     'Quick and healthy vegetable stir fry with a savory sauce', 
     'Quick and healthy vegetable stir fry with a savory sauce.', 
     'Heat oil in a wok. Add vegetables and stir fry for 5 minutes. Add sauce ingredients and cook for another 2 minutes. Serve over rice.', 
     'Bell pepper, Carrot, Broccoli, Soy sauce, Ginger, Garlic, Sesame oil, Rice', 
     654, 28, 126),
    
    ('Chocolate Chip Cookies', 
     'Classic homemade chocolate chip cookies', 
     'Classic homemade chocolate chip cookies - soft, chewy, and delicious.', 
     'Cream butter and sugars. Add eggs and vanilla. Mix in dry ingredients. Fold in chocolate chips. Bake at 350°F for 10-12 minutes.', 
     'Butter, Sugar, Brown sugar, Eggs, Vanilla extract, Flour, Baking soda, Salt, Chocolate chips', 
     1458, 61, 295),
    
    ('Chicken Curry', 
     'Aromatic Indian curry with tender chicken pieces', 
     'Aromatic Indian curry with tender chicken pieces.', 
     'Sauté onions, garlic, and ginger. Add curry powder and cook briefly. Add chicken and cook until browned. Add tomatoes and coconut milk. Simmer until chicken is cooked through.', 
     'Chicken thighs, Onion, Garlic, Ginger, Curry powder, Tomatoes, Coconut milk, Salt, Cilantro', 
     876, 33, 152),
    
    ('Greek Salad', 
     'Fresh and vibrant salad with Mediterranean flavors', 
     'Fresh and vibrant salad with Mediterranean flavors.', 
     'Combine chopped cucumbers, tomatoes, red onion, and olives. Add cubed feta cheese. Dress with olive oil, lemon juice, oregano, salt, and pepper.', 
     'Cucumber, Tomato, Red onion, Kalamata olives, Feta cheese, Olive oil, Lemon juice, Oregano, Salt, Pepper', 
     432, 19, 86),
    
    ('Beef Tacos', 
     'Easy and delicious Mexican-style beef tacos', 
     'Easy and delicious Mexican-style beef tacos.', 
     'Brown ground beef with taco seasoning. Warm tortillas. Assemble tacos with beef, lettuce, tomato, cheese, and sour cream.', 
     'Ground beef, Taco seasoning, Corn tortillas, Lettuce, Tomato, Cheddar cheese, Sour cream, Lime', 
     1023, 47, 211),
    
    ('Banana Bread', 
     'Moist and flavorful banana bread with a hint of cinnamon', 
     'Moist and flavorful banana bread with a hint of cinnamon.', 
     'Mash ripe bananas. Mix with melted butter, sugar, eggs, and vanilla. Fold in flour, baking soda, and salt. Bake at 350°F for 60 minutes.', 
     'Ripe bananas, Butter, Sugar, Eggs, Vanilla extract, Flour, Baking soda, Salt, Cinnamon', 
     765, 31, 140),
    
    ('Mushroom Risotto', 
     'Creamy Italian rice dish with sautéed mushrooms', 
     'Creamy Italian rice dish with sautéed mushrooms.', 
     'Sauté mushrooms and set aside. In the same pan, sauté onions. Add rice and toast briefly. Add white wine. Gradually add hot broth, stirring constantly. Finish with butter and parmesan.', 
     'Arborio rice, Mushrooms, Onion, Garlic, White wine, Vegetable broth, Butter, Parmesan cheese, Salt, Pepper', 
     543, 22, 99),
    
    ('Apple Pie', 
     'Classic American dessert with a flaky crust and cinnamon-spiced apple filling', 
     'Classic American dessert with a flaky crust and cinnamon-spiced apple filling.', 
     'Make pie dough and chill. Slice apples and mix with sugar, cinnamon, and lemon juice. Assemble pie with bottom crust, filling, and top crust. Bake at 375°F for 45-55 minutes.', 
     'Flour, Butter, Sugar, Salt, Apples, Cinnamon, Lemon juice, Egg (for wash)', 
     892, 37, 169);

-- Link recipes to ingredients with quantities
INSERT INTO "RecipeIngredient" (recipe_id, ingredient_id, ingredient_count, amount, description) VALUES
    -- Spaghetti Bolognese
    (1, 8, 1, 500, 'grams, ground'),  -- Beef
    (1, 5, 1, 1, 'large, diced'),     -- Onion
    (1, 4, 3, 3, 'cloves, minced'),   -- Garlic
    (1, 6, 4, 400, 'grams, crushed'), -- Tomatoes
    (1, 10, 1, 500, 'grams'),         -- Pasta
    (1, 3, 1, 2, 'tablespoons'),      -- Olive oil
    (1, 1, 1, 1, 'teaspoon'),         -- Salt
    (1, 2, 1, 1, 'teaspoon'),         -- Pepper
    (1, 18, 1, 1, 'tablespoon'),      -- Oregano
    
    -- Chicken Alfredo
    (2, 7, 2, 500, 'grams, sliced'),  -- Chicken breast
    (2, 10, 1, 500, 'grams'),         -- Pasta
    (2, 15, 1, 250, 'ml, heavy'),     -- Milk (heavy cream)
    (2, 13, 1, 50, 'grams'),          -- Butter
    (2, 16, 1, 100, 'grams, grated'), -- Cheese (parmesan)
    (2, 4, 2, 2, 'cloves, minced'),   -- Garlic
    (2, 1, 1, 1, 'teaspoon'),         -- Salt
    (2, 2, 1, 1, 'teaspoon'),         -- Pepper
    
    -- Vegetable Stir Fry
    (3, 21, 1, 1, 'large, sliced'),   -- Bell pepper
    (3, 22, 2, 2, 'medium, julienned'), -- Carrot
    (3, 26, 1, 3, 'tablespoons'),     -- Soy sauce
    (3, 25, 1, 1, 'tablespoon, grated'), -- Ginger
    (3, 4, 2, 2, 'cloves, minced'),   -- Garlic
    (3, 3, 1, 2, 'tablespoons'),      -- Olive oil (using for sesame oil)
    (3, 9, 1, 2, 'cups, cooked'),     -- Rice
    
    -- Add more recipe ingredients for other recipes...
    -- Chocolate Chip Cookies
    (4, 13, 1, 115, 'grams, softened'), -- Butter
    (4, 12, 1, 150, 'grams'),         -- Sugar
    (4, 14, 2, 2, 'large'),           -- Eggs
    (4, 20, 1, 1, 'teaspoon'),        -- Vanilla extract
    (4, 11, 1, 250, 'grams'),         -- Flour
    (4, 1, 1, 1, 'teaspoon'),         -- Salt
    (4, 28, 1, 200, 'grams, chips'),  -- Chocolate
    
    -- Chicken Curry
    (5, 7, 4, 500, 'grams, cubed'),   -- Chicken (thighs)
    (5, 5, 1, 1, 'large, diced'),     -- Onion
    (5, 4, 3, 3, 'cloves, minced'),   -- Garlic
    (5, 25, 1, 1, 'tablespoon, grated'), -- Ginger
    (5, 6, 2, 2, 'medium, diced'),    -- Tomatoes
    (5, 15, 1, 400, 'ml, coconut'),   -- Milk (coconut)
    (5, 1, 1, 1, 'teaspoon'),         -- Salt
    
    -- Greek Salad
    (6, 6, 2, 2, 'medium, diced'),    -- Tomatoes
    (6, 5, 1, 1, 'small, sliced'),    -- Onion (red)
    (6, 16, 1, 100, 'grams, cubed'),  -- Cheese (feta)
    (6, 3, 1, 3, 'tablespoons'),      -- Olive oil
    (6, 24, 1, 1, 'juiced'),          -- Lemon
    (6, 18, 1, 1, 'teaspoon'),        -- Oregano
    (6, 1, 1, 1, 'teaspoon'),         -- Salt
    (6, 2, 1, 1, 'teaspoon');         -- Pepper

-- Link recipes to recipe meal
INSERT INTO "RecipeMeal" (recipe_id, meal_id) VALUES
    (1, 3), -- Spaghetti Bolognese - Dinner
    (2, 3), -- Chicken Alfredo - Dinner
    (3, 2), -- Vegetable Stir Fry - Lunch
    (3, 3), -- Vegetable Stir Fry - Dinner
    (4, 4), -- Chocolate Chip Cookies - Dessert
    (4, 5), -- Chocolate Chip Cookies - Snack
    (5, 3), -- Chicken Curry - Dinner
    (6, 2), -- Greek Salad - Lunch
    (6, 6), -- Greek Salad - Appetizer
    (7, 3), -- Beef Tacos - Dinner
    (8, 1), -- Banana Bread - Breakfast
    (8, 5), -- Banana Bread - Snack
    (9, 3), -- Mushroom Risotto - Dinner
    (10, 4); -- Apple Pie - Dessert

-- Link recipes to cuisines
INSERT INTO "RecipeCousine" (recipe_id, cousine_id) VALUES
    (1, 1), -- Spaghetti Bolognese - Italian
    (2, 1), -- Chicken Alfredo - Italian
    (3, 4), -- Vegetable Stir Fry - Chinese
    (4, 6), -- Chocolate Chip Cookies - American
    (5, 5), -- Chicken Curry - Indian
    (6, 10), -- Greek Salad - Greek
    (7, 2), -- Beef Tacos - Mexican
    (8, 6), -- Banana Bread - American
    (9, 1), -- Mushroom Risotto - Italian
    (10, 6); -- Apple Pie - American

-- Insert comments and ratings
INSERT INTO "Comment" (user_id, recipe_id, comment, rating) VALUES
    (1, 1, 'This spaghetti bolognese is excellent! Rich flavor and easy to make.', 5),
    (2, 1, 'Good recipe, but I added more garlic and it was perfect.', 4),
    (3, 1, 'My go-to recipe for bolognese. Simple and delicious.', 5),
    (4, 2, 'Creamy and satisfying. Will make again!', 5),
    (5, 2, 'A bit too rich for my taste but my family loved it.', 4),
    (1, 3, 'Great way to use up leftover vegetables.', 4),
    (2, 3, 'Added tofu for protein and it was fantastic.', 5),
    (3, 4, 'Perfect cookie recipe! Crisp edges and chewy center.', 5),
    (4, 4, 'I reduced the sugar by 1/4 cup and they were still delicious.', 4),
    (5, 4, 'Best chocolate chip cookies I\''ve ever made!', 5),
    (1, 5, 'Authentic flavors and easy to adjust spice level.', 5),
    (2, 6, 'Fresh and simple. Perfect summer salad.', 5),
    (3, 7, 'My kids love taco night with this recipe.', 4),
    (4, 8, 'I add walnuts to this recipe. So moist and flavorful!', 5),
    (5, 9, 'Restaurant quality risotto at home. Worth the effort.', 5),
    (1, 10, 'Just like grandma used to make. Delicious!', 5);

-- Create recipe collections
INSERT INTO "RecipeCollection" (user_id, collection_name) VALUES
    (1, 'Weeknight Favorites'),
    (1, 'Dinner Party Ideas'),
    (2, 'Quick Lunches'),
    (3, 'Italian Classics'),
    (4, 'Healthy Options'),
    (5, 'Comfort Food');

-- Link recipes to collections
INSERT INTO "RecipeCollectionRecipe" (recipe_id, collection_id) VALUES
    (1, 1), -- Spaghetti Bolognese in Weeknight Favorites
    (2, 1), -- Chicken Alfredo in Weeknight Favorites
    (7, 1), -- Beef Tacos in Weeknight Favorites
    
    (1, 2), -- Spaghetti Bolognese in Dinner Party Ideas
    (5, 2), -- Chicken Curry in Dinner Party Ideas
    (9, 2), -- Mushroom Risotto in Dinner Party Ideas
    (10, 2), -- Apple Pie in Dinner Party Ideas
    
    (3, 3), -- Vegetable Stir Fry in Quick Lunches
    (6, 3), -- Greek Salad in Quick Lunches
    
    (1, 4), -- Spaghetti Bolognese in Italian Classics
    (2, 4), -- Chicken Alfredo in Italian Classics
    (9, 4), -- Mushroom Risotto in Italian Classics
    
    (3, 5), -- Vegetable Stir Fry in Healthy Options
    (6, 5), -- Greek Salad in Healthy Options
    
    (2, 6), -- Chicken Alfredo in Comfort Food
    (4, 6), -- Chocolate Chip Cookies in Comfort Food
    (8, 6), -- Banana Bread in Comfort Food
    (10, 6); -- Apple Pie in Comfort Food

-- Update recipe ratings based on comments
UPDATE "Recipe" 
SET ratings = (
    SELECT COUNT(*) FROM "Comment" WHERE "Comment".recipe_id = "Recipe".id
),
total_rating = (
    SELECT COALESCE(SUM(rating), 0) FROM "Comment" WHERE "Comment".recipe_id = "Recipe".id
)
WHERE EXISTS (
    SELECT 1 FROM "Comment" WHERE "Comment".recipe_id = "Recipe".id
);

-- Meal name
INSERT INTO "MealName" (name) VALUES
    ('Breakfast'),
    ('Lunch'),
    ('Dinner'),
    ('Sauce'),
    ('Side'),
    ('Appetizer'),
    ('Beverage'),
    ('Snack'),
    ('Dessert');

-- Cousine name
INSERT INTO "CousineName" (name) VALUES
    ('Italian'),
    ('Mexican'),
    ('Chinese'),
    ('Japanese'),
    ('Indian'),
    ('Spanish'),
    ('French'),
    ('German'),
    ('Greek'),
    ('Thai'),
    ('Vietnamese'),
    ('Korean'),
    ('Mediterranean'),
    ('Middle Eastern'),
    ('Caribbean'),
    ('American');

-- Diets for recipes
INSERT INTO "RecipeDiet" (recipe_id, diet_id) VALUES
    (0, 0), (0, 1), (0, 2),
    (1, 3), (1, 4), (1, 5),
    (2, 1), (2, 2), (2, 4),
    (3, 0), (3, 3), (3, 6),
    (4, 2), (4, 3), (4, 5),
    (5, 0), (5, 1), (5, 4),
    (6, 2), (6, 3), (6, 5),
    (7, 0), (7, 1), (7, 6);

-- Diet name
INSERT INTO "DietName" (name) VALUES
    ('Vegetarian'),
    ('Vegan'),
    ('Gluten Free'),
    ('Dairy Free'),
    ('Pescatarian'),
    ('Carnivore'),
    ('Keto');